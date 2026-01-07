use crate::memory::user as mm_user;

use crate::fs::vfs::dispatch::*;
use crate::fs::vfs::error::VfsError;
use crate::syscall::errno;
use crate::task::fd_table::FdType;
use los_hal::print;

/// TEAM_217: struct iovec for writev/readv
#[repr(C)]
#[derive(Clone, Copy)]
struct UserIoVec {
    base: usize,
    len: usize,
}

/// TEAM_217: sys_writev - Vectored write.
/// Required for standard Rust println! efficiency.
pub fn sys_writev(fd: usize, iov_ptr: usize, count: usize) -> i64 {
    if count == 0 {
        return 0;
    }
    if count > 1024 {
        return errno::EINVAL;
    }

    let task = crate::task::current_task();
    let ttbr0 = task.ttbr0;

    // Validate iovec array
    let iov_size = count * core::mem::size_of::<UserIoVec>();
    if mm_user::validate_user_buffer(ttbr0, iov_ptr, iov_size, false).is_err() {
        return errno::EFAULT;
    }

    let mut total_written: i64 = 0;

    for i in 0..count {
        let entry_addr = iov_ptr + i * core::mem::size_of::<UserIoVec>();
        let iov = unsafe {
            let kptr = mm_user::user_va_to_kernel_ptr(ttbr0, entry_addr).unwrap();
            *(kptr as *const UserIoVec)
        };

        if iov.len == 0 {
            continue;
        }

        let res = sys_write(fd, iov.base, iov.len);
        if res < 0 {
            if total_written == 0 {
                return res;
            } else {
                return total_written;
            }
        }
        total_written += res;
    }

    total_written
}

/// TEAM_073: sys_write - Write to a file descriptor.
/// TEAM_194: Updated to support writing to tmpfs files.
pub fn sys_write(fd: usize, buf: usize, len: usize) -> i64 {
    let len = len.min(4096);
    let task = crate::task::current_task();

    // TEAM_194: Look up fd type and dispatch accordingly
    let fd_table = task.fd_table.lock();
    let entry = match fd_table.get(fd) {
        Some(e) => e.clone(),
        None => return errno::EBADF,
    };
    drop(fd_table);

    let ttbr0 = task.ttbr0;

    match entry.fd_type {
        FdType::Stdout | FdType::Stderr => {
            // TEAM_226: Write to console using safe copy
            if mm_user::validate_user_buffer(ttbr0, buf, len, false).is_err() {
                return errno::EFAULT;
            }
            // Copy bytes through kernel-accessible pointers
            let mut kbuf = alloc::vec![0u8; len];
            for i in 0..len {
                if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, buf + i) {
                    kbuf[i] = unsafe { *ptr };
                } else {
                    return errno::EFAULT;
                }
            }
            if let Ok(s) = core::str::from_utf8(&kbuf) {
                print!("{}", s);
            } else {
                for byte in &kbuf {
                    print!("{:02x}", byte);
                }
            }
            len as i64
        }
        FdType::VfsFile(ref file) => {
            if mm_user::validate_user_buffer(ttbr0, buf, len, false).is_err() {
                return errno::EFAULT;
            }
            let mut kbuf = alloc::vec![0u8; len];
            for i in 0..len {
                if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, buf + i) {
                    kbuf[i] = unsafe { *ptr };
                } else {
                    return errno::EFAULT;
                }
            }
            match vfs_write(file, &kbuf) {
                Ok(n) => n as i64,
                Err(VfsError::NoSpace) => -28,      // ENOSPC
                Err(VfsError::FileTooLarge) => -27, // EFBIG
                Err(_) => errno::EIO,
            }
        }
        _ => errno::EBADF,
    }
}
