use crate::fs::vfs::dispatch::*;
// TEAM_413: Use new syscall helpers
use crate::syscall::{Stat, get_fd, write_struct_to_user};
use crate::task::fd_table::FdType;

/// TEAM_168: sys_fstat - Get file status.
/// TEAM_258: Updated to use Stat constructors for architecture independence.
/// TEAM_413: Updated to use write_struct_to_user helper.
pub fn sys_fstat(fd: usize, stat_buf: usize) -> i64 {
    let task = crate::task::current_task();

    // TEAM_413: Use get_fd helper
    let entry = match get_fd(fd) {
        Ok(e) => e,
        Err(e) => return e,
    };

    let stat = match &entry.fd_type {
        // TEAM_258: Use constructor for architecture independence
        FdType::Stdin | FdType::Stdout | FdType::Stderr => {
            Stat::new_device(crate::fs::mode::S_IFCHR | 0o666, 0)
        }
        FdType::VfsFile(file) => match vfs_fstat(file) {
            Ok(s) => s,
            Err(e) => return e.into(), // TEAM_413: Use From<VfsError> for i64
        },
        // TEAM_258: Use constructor for architecture independence
        FdType::PipeRead(_) | FdType::PipeWrite(_) => {
            Stat::new_pipe(crate::fs::pipe::PIPE_BUF_SIZE as i32)
        }
        // TEAM_258: Use constructor for architecture independence
        FdType::PtyMaster(_) | FdType::PtySlave(_) => {
            Stat::new_device(crate::fs::mode::S_IFCHR | 0o666, 0)
        }
        // TEAM_394: Epoll and EventFd are anonymous inodes
        FdType::Epoll(_) | FdType::EventFd(_) => {
            Stat::new_device(crate::fs::mode::S_IFCHR | 0o600, 0)
        }
    };

    // TEAM_413: Use write_struct_to_user helper
    match write_struct_to_user(task.ttbr0, stat_buf, &stat) {
        Ok(()) => 0,
        Err(e) => e,
    }
}

/// TEAM_409: sys_fstatat - Get file status relative to directory fd.
/// Signature: fstatat(dirfd, pathname, statbuf, flags)
///
/// This is the "at" variant of stat, supporting AT_FDCWD for current directory.
/// TEAM_413: Updated to use resolve_at_path and write_struct_to_user helpers.
pub fn sys_fstatat(dirfd: i32, pathname: usize, stat_buf: usize, _flags: i32) -> i64 {
    use crate::syscall::{resolve_at_path, write_struct_to_user};

    let task = crate::task::current_task();

    // TEAM_413: Use resolve_at_path helper for pathname resolution
    let path_str = match resolve_at_path(dirfd, pathname) {
        Ok(s) => s,
        Err(e) => return e,
    };

    // Get file status via VFS
    let stat = match vfs_stat(&path_str) {
        Ok(s) => s,
        Err(e) => return e.into(), // TEAM_413: Use From<VfsError> for i64
    };

    // TEAM_413: Use write_struct_to_user helper
    match write_struct_to_user(task.ttbr0, stat_buf, &stat) {
        Ok(()) => 0,
        Err(e) => e,
    }
}
