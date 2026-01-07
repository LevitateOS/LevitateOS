//! Filesystem operations
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
use crate::sysno::{
    SYS_DUP, SYS_DUP3, SYS_FSTAT, SYS_GETCWD, SYS_GETDENTS, SYS_LINKAT, SYS_MKDIRAT, SYS_OPENAT,
    SYS_PIPE2, SYS_READLINKAT, SYS_RENAMEAT, SYS_SYMLINKAT, SYS_UNLINKAT, SYS_UTIMENSAT,
};
use crate::time::Timespec;

/// TEAM_250: Open flags for suite_test_core
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 64;
pub const O_EXCL: u32 = 128;
pub const O_TRUNC: u32 = 512;
pub const O_APPEND: u32 = 1024;
pub const O_CLOEXEC: u32 = 0x80000;

/// TEAM_217: Linux-compatible Stat structure (128 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad1: u64,
    pub st_size: i64,
    pub st_blksize: i32,
    pub __pad2: i32,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: u64,
    pub st_mtime: i64,
    pub st_mtime_nsec: u64,
    pub st_ctime: i64,
    pub st_ctime_nsec: u64,
    pub __unused: [u32; 2],
}

/// TEAM_176: Dirent64 structure for directory entries.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    // d_name follows
}

/// TEAM_176: File type constants for d_type field.
pub mod d_type {
    pub const DT_UNKNOWN: u8 = 0;
    pub const DT_FIFO: u8 = 1;
    pub const DT_CHR: u8 = 2;
    pub const DT_DIR: u8 = 4;
    pub const DT_BLK: u8 = 6;
    pub const DT_REG: u8 = 8;
    pub const DT_LNK: u8 = 10;
    pub const DT_SOCK: u8 = 12;
}

/// TEAM_168: Open a file.
#[inline]
pub fn openat(path: &str, flags: u32) -> isize {
    arch::syscall3(
        SYS_OPENAT,
        path.as_ptr() as u64,
        path.len() as u64,
        flags as u64,
    ) as isize
}

/// TEAM_168: Get file status.
#[inline]
pub fn fstat(fd: usize, stat: &mut Stat) -> isize {
    arch::syscall2(SYS_FSTAT, fd as u64, stat as *mut Stat as u64) as isize
}

/// TEAM_176: Read directory entries.
#[inline]
pub fn getdents(fd: usize, buf: &mut [u8]) -> isize {
    arch::syscall3(
        SYS_GETDENTS,
        fd as u64,
        buf.as_mut_ptr() as u64,
        buf.len() as u64,
    ) as isize
}

/// TEAM_192: Get current working directory.
#[inline]
pub fn getcwd(buf: &mut [u8]) -> isize {
    arch::syscall2(SYS_GETCWD, buf.as_mut_ptr() as u64, buf.len() as u64) as isize
}

/// TEAM_192: Create directory.
#[inline]
pub fn mkdirat(dfd: i32, path: &str, mode: u32) -> isize {
    arch::syscall4(
        SYS_MKDIRAT,
        dfd as u64,
        path.as_ptr() as u64,
        path.len() as u64,
        mode as u64,
    ) as isize
}

/// TEAM_192: Remove file or directory.
#[inline]
pub fn unlinkat(dfd: i32, path: &str, flags: u32) -> isize {
    arch::syscall4(
        SYS_UNLINKAT,
        dfd as u64,
        path.as_ptr() as u64,
        path.len() as u64,
        flags as u64,
    ) as isize
}

/// TEAM_192: Rename/move file or directory.
#[inline]
pub fn renameat(old_dfd: i32, old_path: &str, new_dfd: i32, new_path: &str) -> isize {
    arch::syscall6(
        SYS_RENAMEAT,
        old_dfd as u64,
        old_path.as_ptr() as u64,
        old_path.len() as u64,
        new_dfd as u64,
        new_path.as_ptr() as u64,
        new_path.len() as u64,
    ) as isize
}

/// TEAM_198: Create a symbolic link.
#[inline]
pub fn symlinkat(target: &str, linkdirfd: i32, linkpath: &str) -> isize {
    arch::syscall5(
        SYS_SYMLINKAT,
        target.as_ptr() as u64,
        target.len() as u64,
        linkdirfd as u64,
        linkpath.as_ptr() as u64,
        linkpath.len() as u64,
    ) as isize
}

/// TEAM_253: Read value of a symbolic link.
#[inline]
pub fn readlinkat(dirfd: i32, path: &str, buf: &mut [u8]) -> isize {
    arch::syscall5(
        SYS_READLINKAT,
        dirfd as u64,
        path.as_ptr() as u64,
        path.len() as u64,
        buf.as_mut_ptr() as u64,
        buf.len() as u64,
    ) as isize
}

/// TEAM_209: Create a hard link.
#[inline]
pub fn linkat(olddfd: i32, oldpath: &str, newdfd: i32, newpath: &str, flags: u32) -> isize {
    arch::syscall7(
        SYS_LINKAT,
        olddfd as u64,
        oldpath.as_ptr() as u64,
        oldpath.len() as u64,
        newdfd as u64,
        newpath.as_ptr() as u64,
        newpath.len() as u64,
        flags as u64,
    ) as isize
}

/// TEAM_198: UTIME_NOW - set to current time
pub const UTIME_NOW: u64 = 0x3FFFFFFF;
/// TEAM_198: UTIME_OMIT - don't change
pub const UTIME_OMIT: u64 = 0x3FFFFFFE;

/// TEAM_198: Set file access and modification times.
#[inline]
pub fn utimensat(dirfd: i32, path: &str, times: Option<&[Timespec; 2]>, flags: u32) -> isize {
    let times_ptr = match times {
        Some(t) => t.as_ptr() as u64,
        None => 0,
    };
    arch::syscall5(
        SYS_UTIMENSAT,
        dirfd as u64,
        path.as_ptr() as u64,
        path.len() as u64,
        times_ptr,
        flags as u64,
    ) as isize
}

/// TEAM_233: Create a pipe.
#[inline]
pub fn pipe2(pipefd: &mut [i32; 2], flags: u32) -> isize {
    arch::syscall2(SYS_PIPE2, pipefd.as_mut_ptr() as u64, flags as u64) as isize
}

/// TEAM_233: Duplicate a file descriptor to lowest available.
#[inline]
pub fn dup(oldfd: usize) -> isize {
    arch::syscall1(SYS_DUP, oldfd as u64) as isize
}

/// TEAM_233: Duplicate a file descriptor to a specific number.
#[inline]
pub fn dup2(oldfd: usize, newfd: usize) -> isize {
    dup3(oldfd, newfd, 0)
}

/// TEAM_233: Duplicate a file descriptor with flags.
#[inline]
pub fn dup3(oldfd: usize, newfd: usize, flags: u32) -> isize {
    arch::syscall3(SYS_DUP3, oldfd as u64, newfd as u64, flags as u64) as isize
}
