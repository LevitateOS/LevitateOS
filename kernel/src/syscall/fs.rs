//! TEAM_171: File system system calls.

use crate::syscall::{Stat, errno, errno_file, write_to_user_buf};
use los_hal::print;

/// TEAM_081: sys_read - Read from a file descriptor.
pub fn sys_read(fd: usize, buf: usize, len: usize) -> i64 {
    if fd != 0 {
        return errno::EBADF;
    }

    let max_read = len.min(4096);
    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, buf, max_read, true).is_err() {
        return errno::EFAULT;
    }

    if len == 0 {
        return 0;
    }

    let mut bytes_read = 0usize;
    let ttbr0 = task.ttbr0;

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

/// TEAM_073: sys_write - Write to a file descriptor.
pub fn sys_write(fd: usize, buf: usize, len: usize) -> i64 {
    if fd != 1 && fd != 2 {
        return errno::EBADF;
    }

    let len = len.min(4096);
    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, buf, len, false).is_err() {
        return errno::EFAULT;
    }

    let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    if let Ok(s) = core::str::from_utf8(slice) {
        print!("{}", s);
    } else {
        for byte in slice {
            print!("{:02x}", byte);
        }
    }

    len as i64
}

/// TEAM_168: sys_openat - Open a file from initramfs.
pub fn sys_openat(path: usize, path_len: usize, _flags: u32) -> i64 {
    if path_len == 0 || path_len > 256 {
        return errno::EINVAL;
    }

    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, path, path_len, false).is_err() {
        return errno::EFAULT;
    }

    let mut path_buf = [0u8; 256];
    for i in 0..path_len {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, path + i) {
            path_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }

    let path_str = match core::str::from_utf8(&path_buf[..path_len]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    let initramfs_guard = crate::fs::INITRAMFS.lock();
    let initramfs = match initramfs_guard.as_ref() {
        Some(i) => i,
        None => return errno_file::ENOENT,
    };

    let lookup_path = path_str.trim_start_matches('/');
    let file_index = {
        let mut found_idx = None;
        for (idx, entry) in initramfs.iter().enumerate() {
            let entry_name = entry.name.trim_start_matches('/');
            if entry_name == lookup_path {
                found_idx = Some(idx);
                break;
            }
        }
        match found_idx {
            Some(idx) => idx,
            None => return errno_file::ENOENT,
        }
    };

    drop(initramfs_guard);

    let mut fd_table = task.fd_table.lock();
    match fd_table.alloc(crate::task::fd_table::FdType::InitramfsFile {
        file_index,
        offset: 0,
    }) {
        Some(fd) => fd as i64,
        None => errno_file::EMFILE,
    }
}

/// TEAM_168: sys_close - Close a file descriptor.
pub fn sys_close(fd: usize) -> i64 {
    let task = crate::task::current_task();
    let mut fd_table = task.fd_table.lock();

    if fd < 3 {
        return errno::EINVAL;
    }

    if fd_table.close(fd) { 0 } else { errno::EBADF }
}

/// TEAM_168: sys_fstat - Get file status.
pub fn sys_fstat(fd: usize, stat_buf: usize) -> i64 {
    let task = crate::task::current_task();
    let stat_size = core::mem::size_of::<Stat>();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, stat_buf, stat_size, true).is_err() {
        return errno::EFAULT;
    }

    let fd_table = task.fd_table.lock();
    let entry = match fd_table.get(fd) {
        Some(e) => e,
        None => return errno::EBADF,
    };

    let stat = match entry.fd_type {
        crate::task::fd_table::FdType::Stdin
        | crate::task::fd_table::FdType::Stdout
        | crate::task::fd_table::FdType::Stderr => Stat {
            st_size: 0,
            st_mode: 2,
            _pad: 0,
        },
        crate::task::fd_table::FdType::InitramfsFile { file_index, .. } => {
            let initramfs_guard = crate::fs::INITRAMFS.lock();
            let initramfs = match initramfs_guard.as_ref() {
                Some(i) => i,
                None => return errno::EBADF,
            };

            let file_size = initramfs
                .iter()
                .nth(file_index)
                .map(|e| e.data.len())
                .unwrap_or(0);

            Stat {
                st_size: file_size as u64,
                st_mode: 1,
                _pad: 0,
            }
        }
    };

    let stat_bytes =
        unsafe { core::slice::from_raw_parts(&stat as *const Stat as *const u8, stat_size) };

    for (i, &byte) in stat_bytes.iter().enumerate() {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, stat_buf + i) {
            unsafe { *ptr = byte };
        } else {
            return errno::EFAULT;
        }
    }

    0
}
