use crate::memory::user as mm_user;

use crate::fs::vfs::dispatch::*;
use crate::fs::vfs::error::VfsError;
use crate::syscall::{errno, write_to_user_buf};
use crate::task::fd_table::FdType;

/// TEAM_217: sys_readv - Vectored read.
pub fn sys_readv(fd: usize, iov_ptr: usize, count: usize) -> i64 {
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

    let mut total_read: i64 = 0;

    for i in 0..count {
        let entry_addr = iov_ptr + i * core::mem::size_of::<UserIoVec>();
        let iov = unsafe {
            let kptr = mm_user::user_va_to_kernel_ptr(ttbr0, entry_addr).unwrap();
            *(kptr as *const UserIoVec)
        };

        if iov.len == 0 {
            continue;
        }

        let res = sys_read(fd, iov.base, iov.len);
        if res < 0 {
            if total_read == 0 {
                return res;
            } else {
                return total_read;
            }
        }
        total_read += res;
        if res < iov.len as i64 {
            // Short read, stop here
            break;
        }
    }

    total_read
}

/// TEAM_217: struct iovec for writev/readv
#[repr(C)]
#[derive(Clone, Copy)]
struct UserIoVec {
    base: usize,
    len: usize,
}

/// TEAM_081: sys_read - Read from a file descriptor.
/// TEAM_178: Refactored to dispatch by fd type, added InitramfsFile support.
pub fn sys_read(fd: usize, buf: usize, len: usize) -> i64 {
    if len == 0 {
        return 0;
    }

    let task = crate::task::current_task();

    // TEAM_178: Look up fd type and dispatch accordingly
    let fd_table = task.fd_table.lock();
    let entry = match fd_table.get(fd) {
        Some(e) => e.clone(),
        None => return errno::EBADF,
    };
    drop(fd_table);

    let ttbr0 = task.ttbr0;

    match entry.fd_type {
        FdType::Stdin => read_stdin(buf, len, ttbr0),
        FdType::VfsFile(ref file) => {
            if mm_user::validate_user_buffer(ttbr0, buf, len, true).is_err() {
                return errno::EFAULT;
            }
            let mut kbuf = alloc::vec![0u8; len];
            match vfs_read(file, &mut kbuf) {
                Ok(n) => {
                    for i in 0..n {
                        if !write_to_user_buf(ttbr0, buf, i, kbuf[i]) {
                            return errno::EFAULT;
                        }
                    }
                    n as i64
                }
                Err(VfsError::BadFd) => errno::EBADF,
                Err(_) => errno::EIO,
            }
        }
        _ => errno::EBADF,
    }
}

/// TEAM_178: Read from stdin (keyboard/console input).
fn read_stdin(buf: usize, len: usize, ttbr0: usize) -> i64 {
    let max_read = len.min(4096);
    if mm_user::validate_user_buffer(ttbr0, buf, max_read, true).is_err() {
        return errno::EFAULT;
    }

    let mut bytes_read = 0usize;

    loop {
        poll_input_devices(ttbr0, buf, &mut bytes_read, max_read);
        if bytes_read > 0 {
            break;
        }

        unsafe {
            los_hal::interrupts::enable();
        }
        let _ = los_hal::interrupts::disable();

        crate::task::yield_now();
    }

    bytes_read as i64
}

fn poll_input_devices(ttbr0: usize, user_buf: usize, bytes_read: &mut usize, max_read: usize) {
    crate::input::poll();

    while *bytes_read < max_read {
        if let Some(ch) = crate::input::read_char() {
            if ch == '\x03' {
                // Ctrl+C received - signal foreground process
                crate::syscall::signal::signal_foreground_process(crate::syscall::signal::SIGINT);
                // Don't write to buffer, don't return to user
                continue;
            }
            if !write_to_user_buf(ttbr0, user_buf, *bytes_read, ch as u8) {
                return;
            }
            *bytes_read += 1;
            if ch == '\n' {
                return;
            }
        } else {
            break;
        }
    }

    if *bytes_read < max_read {
        while let Some(byte) = los_hal::console::read_byte() {
            let byte = if byte == b'\r' { b'\n' } else { byte };
            if !write_to_user_buf(ttbr0, user_buf, *bytes_read, byte) {
                return;
            }
            *bytes_read += 1;
            if byte == b'\n' {
                return;
            }
            if *bytes_read >= max_read {
                return;
            }
        }
    }
}
