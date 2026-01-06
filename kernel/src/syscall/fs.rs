use crate::fs::vfs::dispatch::*;
use crate::fs::vfs::error::VfsError;
use crate::fs::vfs::file::OpenFlags;
use crate::syscall::{Stat, errno, errno_file, write_to_user_buf};
use crate::task::fd_table::FdType;
use core::convert::TryFrom;
use los_hal::print;

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
            if crate::task::user_mm::validate_user_buffer(ttbr0, buf, len, true).is_err() {
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
    if crate::task::user_mm::validate_user_buffer(ttbr0, buf, max_read, true).is_err() {
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

/// TEAM_178: Read from an initramfs file.
// read_initramfs_file REMOVED

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
            // Write to console
            if crate::task::user_mm::validate_user_buffer(ttbr0, buf, len, false).is_err() {
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
        FdType::VfsFile(ref file) => {
            if crate::task::user_mm::validate_user_buffer(ttbr0, buf, len, false).is_err() {
                return errno::EFAULT;
            }
            let mut kbuf = alloc::vec![0u8; len];
            for i in 0..len {
                if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(ttbr0, buf + i) {
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

/// TEAM_168: sys_openat - Open a file from initramfs.
/// TEAM_176: Updated to support opening directories for getdents.
/// TEAM_194: Updated to support tmpfs at /tmp with O_CREAT and O_TRUNC.
pub fn sys_openat(path: usize, path_len: usize, flags: u32) -> i64 {
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

    // TEAM_205: All paths now go through generic vfs_open
    let vfs_flags = OpenFlags::new(flags);
    match vfs_open(path_str, vfs_flags, 0o666) {
        Ok(file) => {
            let mut fd_table = task.fd_table.lock();
            match fd_table.alloc(FdType::VfsFile(file)) {
                Some(fd) => fd as i64,
                None => errno_file::EMFILE,
            }
        }
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(VfsError::AlreadyExists) => errno_file::EEXIST,
        Err(VfsError::NotADirectory) => errno_file::ENOTDIR,
        Err(VfsError::IsADirectory) => {
            // If it's a directory, we need to handle it specifically for getdents if needed
            // Actually vfs_open handles directories if flags allow.
            // But let's check for O_DIRECTORY or similar if we care.
            // For now, let's just retry opening as directory if needed.
            // Wait, vfs_open already returns a File reflecting the directory if it's one.
            // So we are done.
            errno_file::EIO // Should not normally happen if vfs_open succeeded
        }
        Err(_) => errno_file::EIO,
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
        // TEAM_201: Updated to use extended Stat struct
        crate::task::fd_table::FdType::Stdin
        | crate::task::fd_table::FdType::Stdout
        | crate::task::fd_table::FdType::Stderr => Stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: crate::fs::mode::S_IFCHR | 0o666,
            st_nlink: 1,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_atime_nsec: 0,
            st_mtime: 0,
            st_mtime_nsec: 0,
            st_ctime: 0,
            st_ctime_nsec: 0,
        },
        FdType::VfsFile(ref file) => match vfs_fstat(file) {
            Ok(s) => s,
            Err(_) => return errno::EBADF,
        },
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

// TEAM_176: Dirent64 structure for getdents syscall.
// Matches Linux ABI layout.
#[repr(C, packed)]
struct Dirent64 {
    d_ino: u64,    // Inode number
    d_off: i64,    // Offset to next entry
    d_reclen: u16, // Length of this record
    d_type: u8,    // File type
                   // d_name follows (null-terminated)
}

pub fn sys_getdents(fd: usize, buf: usize, buf_len: usize) -> i64 {
    if buf_len == 0 {
        return 0;
    }

    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, buf, buf_len, true).is_err() {
        return errno::EFAULT;
    }

    let fd_table = task.fd_table.lock();
    let entry = match fd_table.get(fd) {
        Some(e) => e.clone(),
        None => return errno::EBADF,
    };
    drop(fd_table);

    match entry.fd_type {
        FdType::VfsFile(ref file) => {
            let mut bytes_written = 0usize;
            loop {
                let offset = file.tell() as usize;
                match vfs_readdir(file, offset) {
                    Ok(Some(entry)) => {
                        let name_bytes = entry.name.as_bytes();
                        let name_len = name_bytes.len();
                        let reclen = ((19 + name_len + 1 + 7) / 8) * 8;

                        if bytes_written + reclen > buf_len {
                            break;
                        }

                        let dtype = match entry.file_type {
                            crate::fs::mode::S_IFDIR => 4,
                            crate::fs::mode::S_IFREG => 8,
                            crate::fs::mode::S_IFLNK => 10,
                            _ => 0,
                        };

                        let dirent = Dirent64 {
                            d_ino: entry.ino,
                            d_off: (offset + 1) as i64,
                            d_reclen: reclen as u16,
                            d_type: dtype,
                        };

                        let dirent_bytes = unsafe {
                            core::slice::from_raw_parts(
                                &dirent as *const Dirent64 as *const u8,
                                core::mem::size_of::<Dirent64>(),
                            )
                        };

                        for (i, &byte) in dirent_bytes.iter().enumerate() {
                            if !write_to_user_buf(task.ttbr0, buf, bytes_written + i, byte) {
                                return errno::EFAULT;
                            }
                        }

                        let name_offset = bytes_written + core::mem::size_of::<Dirent64>();
                        for (i, &byte) in name_bytes.iter().enumerate() {
                            if !write_to_user_buf(task.ttbr0, buf, name_offset + i, byte) {
                                return errno::EFAULT;
                            }
                        }

                        if !write_to_user_buf(task.ttbr0, buf, name_offset + name_len, 0) {
                            return errno::EFAULT;
                        }

                        let _ =
                            file.seek((offset + 1) as i64, crate::fs::vfs::ops::SeekWhence::Set);
                        bytes_written += reclen;
                    }
                    Ok(None) => break,
                    Err(_) => return errno::EBADF,
                }
            }
            bytes_written as i64
        }
        _ => errno_file::ENOTDIR,
    }
}

pub fn sys_getcwd(buf: usize, size: usize) -> i64 {
    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, buf, size, true).is_err() {
        return errno::EFAULT;
    }

    let path = "/";
    let path_len = path.len();
    if size < path_len + 1 {
        return -34; // ERANGE
    }

    for (i, &byte) in path.as_bytes().iter().enumerate() {
        if !write_to_user_buf(task.ttbr0, buf, i, byte) {
            return errno::EFAULT;
        }
    }
    if !write_to_user_buf(task.ttbr0, buf, path_len, 0) {
        return errno::EFAULT;
    }

    (path_len + 1) as i64
}

/// TEAM_192: sys_mkdirat - Create directory.
/// TEAM_194: Updated to support tmpfs at /tmp.
pub fn sys_mkdirat(_dfd: i32, path: usize, path_len: usize, _mode: u32) -> i64 {
    if path_len == 0 || path_len > 256 {
        return errno::EINVAL;
    }

    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, path, path_len, false).is_err() {
        return errno::EFAULT;
    }

    // Read path from userspace
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

    match vfs_mkdir(path_str, _mode) {
        Ok(()) => 0,
        Err(VfsError::AlreadyExists) => -17, // EEXIST
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(VfsError::NotADirectory) => errno_file::ENOTDIR,
        Err(_) => errno::EINVAL,
    }
}

/// TEAM_194: AT_REMOVEDIR flag for unlinkat
const AT_REMOVEDIR: u32 = 0x200;

/// TEAM_192: sys_unlinkat - Remove file or directory.
/// TEAM_194: Updated to support tmpfs at /tmp.
pub fn sys_unlinkat(_dfd: i32, path: usize, path_len: usize, flags: u32) -> i64 {
    if path_len == 0 || path_len > 256 {
        return errno::EINVAL;
    }

    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, path, path_len, false).is_err() {
        return errno::EFAULT;
    }

    // Read path from userspace
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

    let res = if (flags & AT_REMOVEDIR) != 0 {
        vfs_rmdir(path_str)
    } else {
        vfs_unlink(path_str)
    };

    match res {
        Ok(()) => 0,
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(VfsError::NotADirectory) => errno_file::ENOTDIR,
        Err(VfsError::DirectoryNotEmpty) => -39, // ENOTEMPTY
        Err(_) => errno::EINVAL,
    }
}

/// TEAM_192: sys_renameat - Rename or move file or directory.
/// TEAM_194: Updated to support tmpfs at /tmp.
pub fn sys_renameat(
    _old_dfd: i32,
    old_path: usize,
    old_path_len: usize,
    _new_dfd: i32,
    new_path: usize,
    new_path_len: usize,
) -> i64 {
    if old_path_len == 0 || old_path_len > 256 || new_path_len == 0 || new_path_len > 256 {
        return errno::EINVAL;
    }

    let task = crate::task::current_task();

    // Validate and read old path
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, old_path, old_path_len, false)
        .is_err()
    {
        return errno::EFAULT;
    }
    let mut old_path_buf = [0u8; 256];
    for i in 0..old_path_len {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, old_path + i) {
            old_path_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    let old_path_str = match core::str::from_utf8(&old_path_buf[..old_path_len]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    // Validate and read new path
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, new_path, new_path_len, false)
        .is_err()
    {
        return errno::EFAULT;
    }
    let mut new_path_buf = [0u8; 256];
    for i in 0..new_path_len {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, new_path + i) {
            new_path_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    let new_path_str = match core::str::from_utf8(&new_path_buf[..new_path_len]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    match vfs_rename(old_path_str, new_path_str) {
        Ok(()) => 0,
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(VfsError::NotADirectory) => errno_file::ENOTDIR,
        Err(VfsError::CrossDevice) => -18, // EXDEV
        Err(_) => errno::EINVAL,
    }
}

/// TEAM_198: UTIME_NOW constant - set time to current time
const UTIME_NOW: u64 = 0x3FFFFFFF;
/// TEAM_198: UTIME_OMIT constant - don't change time
const UTIME_OMIT: u64 = 0x3FFFFFFE;

/// TEAM_198: sys_utimensat - Set file access and modification times.
///
/// # Arguments
/// * `_dirfd` - Directory fd (AT_FDCWD for cwd) - currently ignored
/// * `path` - Path to file
/// * `path_len` - Length of path
/// * `times` - Pointer to [atime, mtime] timespec array (0 = use current time)
/// * `_flags` - AT_SYMLINK_NOFOLLOW - currently ignored
pub fn sys_utimensat(_dirfd: i32, path: usize, path_len: usize, times: usize, _flags: u32) -> i64 {
    let task = crate::task::current_task();
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

    // Get current time
    let now = crate::syscall::time::uptime_seconds();

    // Determine new atime and mtime
    let (atime, mtime) = if times == 0 {
        (Some(now), Some(now))
    } else {
        // struct timespec { u64 tv_sec; u64 tv_nsec; }
        let mut times_data = [0u64; 4]; // [atime_sec, atime_nsec, mtime_sec, mtime_nsec]
        for i in 0..4 {
            let mut val = 0u64;
            for j in 0..8 {
                if let Some(ptr) =
                    crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, times + i * 16 + j)
                {
                    val |= (unsafe { *ptr } as u64) << (j * 8);
                } else {
                    return errno::EFAULT;
                }
            }
            times_data[i] = val;
        }

        let atime = if times_data[1] == UTIME_OMIT {
            None
        } else if times_data[1] == UTIME_NOW {
            Some(now)
        } else {
            Some(times_data[0])
        };
        let mtime = if times_data[3] == UTIME_OMIT {
            None
        } else if times_data[3] == UTIME_NOW {
            Some(now)
        } else {
            Some(times_data[2])
        };
        (atime, mtime)
    };

    vfs_utimes(path_str, atime, mtime)
        .map(|_| 0)
        .unwrap_or_else(|e| e.to_errno())
}

/// TEAM_198: sys_symlinkat - Create a symbolic link.
///
/// # Arguments
/// * `target` - Target path the symlink points to
/// * `target_len` - Length of target
/// * `_linkdirfd` - Directory fd for link path (ignored, use AT_FDCWD)
/// * `linkpath` - Path for the new symlink
/// * `linkpath_len` - Length of link path
pub fn sys_symlinkat(
    target: usize,
    target_len: usize,
    _linkdirfd: i32,
    linkpath: usize,
    linkpath_len: usize,
) -> i64 {
    let task = crate::task::current_task();
    let mut target_buf = [0u8; 256];
    for i in 0..target_len {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, target + i) {
            target_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    let target_str = match core::str::from_utf8(&target_buf[..target_len]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    let mut linkpath_buf = [0u8; 256];
    for i in 0..linkpath_len {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, linkpath + i) {
            linkpath_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    let linkpath_str = match core::str::from_utf8(&linkpath_buf[..linkpath_len]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    match vfs_symlink(target_str, linkpath_str) {
        Ok(()) => 0,
        Err(VfsError::AlreadyExists) => -17, // EEXIST
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(VfsError::NotADirectory) => errno_file::ENOTDIR,
        Err(_) => errno::EINVAL,
    }
}

/// TEAM_204: sys_readlinkat - Read value of a symbolic link.
pub fn sys_readlinkat(dirfd: i32, path: usize, path_len: usize, buf: usize, buf_len: usize) -> i64 {
    if dirfd != -100 {
        // AT_FDCWD
        return errno::ENOSYS;
    }

    let task = crate::task::current_task();
    let mut path_buf = [0u8; 256];
    for i in 0..path_len.min(256) {
        if let Some(ptr) = crate::task::user_mm::user_va_to_kernel_ptr(task.ttbr0, path + i) {
            path_buf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    let path_str = match core::str::from_utf8(&path_buf[..path_len.min(256)]) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    match vfs_readlink(path_str) {
        Ok(target) => {
            let n = target.len().min(buf_len);
            let target_bytes = target.as_bytes();
            for i in 0..n {
                if !write_to_user_buf(task.ttbr0, buf, i, target_bytes[i]) {
                    return errno::EFAULT;
                }
            }
            n as i64
        }
        Err(VfsError::NotFound) => errno_file::ENOENT,
        Err(_) => errno::EIO,
    }
}
// TEAM_206: Mount a filesystem
pub fn sys_mount(
    source_ptr: usize,
    target_ptr: usize,
    fstype_ptr: usize,
    flags: usize,
    _data_ptr: usize,
) -> i64 {
    let task = crate::task::current_task();
    let ttbr0 = task.ttbr0;

    // Read source string
    let source = match crate::syscall::sys::read_user_string(ttbr0, source_ptr, 256) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    // Read target string
    let target = match crate::syscall::sys::read_user_string(ttbr0, target_ptr, 256) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    // Read fstype string
    let fstype_str = match crate::syscall::sys::read_user_string(ttbr0, fstype_ptr, 64) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    // Convert fstype
    let fstype = match crate::fs::mount::FsType::try_from(fstype_str.as_str()) {
        Ok(t) => t,
        Err(_) => return errno::EINVAL,
    };

    // Convert flags (simplified)
    let mount_flags = if (flags & 1) != 0 {
        crate::fs::mount::MountFlags::readonly()
    } else {
        crate::fs::mount::MountFlags::new()
    };

    match crate::fs::mount::mount(
        crate::fs::path::Path::new(&target),
        fstype,
        mount_flags,
        &source,
    ) {
        Ok(_) => 0,
        Err(e) => match e {
            crate::fs::mount::MountError::AlreadyMounted => errno::EEXIST, // or EBUSY
            crate::fs::mount::MountError::NotMounted => errno::EINVAL,
            crate::fs::mount::MountError::InvalidMountpoint => errno::ENOENT,
            crate::fs::mount::MountError::UnsupportedFsType => errno::EINVAL, // or ENODEV
            crate::fs::mount::MountError::PermissionDenied => errno_file::EACCES,
        },
    }
}

// TEAM_206: Unmount a filesystem
pub fn sys_umount(target_ptr: usize, _flags: usize) -> i64 {
    let task = crate::task::current_task();
    let ttbr0 = task.ttbr0;

    // Read target string
    let target = match crate::syscall::sys::read_user_string(ttbr0, target_ptr, 256) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    match crate::fs::mount::umount(crate::fs::path::Path::new(&target)) {
        Ok(_) => 0,
        Err(_) => errno::EINVAL, // Simplified mapping
    }
}
