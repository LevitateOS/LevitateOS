//! Filesystem operations
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
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_OPENAT,
            in("x0") path.as_ptr(),
            in("x1") path.len(),
            in("x2") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_168: Get file status.
#[inline]
pub fn fstat(fd: usize, stat: &mut Stat) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_FSTAT,
            in("x0") fd,
            in("x1") stat as *mut Stat,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_176: Read directory entries.
#[inline]
pub fn getdents(fd: usize, buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETDENTS,
            in("x0") fd,
            in("x1") buf.as_mut_ptr(),
            in("x2") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Get current working directory.
#[inline]
pub fn getcwd(buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETCWD,
            in("x0") buf.as_mut_ptr(),
            in("x1") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Create directory.
#[inline]
pub fn mkdirat(dfd: i32, path: &str, mode: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_MKDIRAT,
            in("x0") dfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") mode,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Remove file or directory.
#[inline]
pub fn unlinkat(dfd: i32, path: &str, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_UNLINKAT,
            in("x0") dfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Rename/move file or directory.
#[inline]
pub fn renameat(old_dfd: i32, old_path: &str, new_dfd: i32, new_path: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_RENAMEAT,
            in("x0") old_dfd,
            in("x1") old_path.as_ptr(),
            in("x2") old_path.len(),
            in("x3") new_dfd,
            in("x4") new_path.as_ptr(),
            in("x5") new_path.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_198: Create a symbolic link.
#[inline]
pub fn symlinkat(target: &str, linkdirfd: i32, linkpath: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SYMLINKAT,
            in("x0") target.as_ptr(),
            in("x1") target.len(),
            in("x2") linkdirfd,
            in("x3") linkpath.as_ptr(),
            in("x4") linkpath.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_253: Read value of a symbolic link.
#[inline]
pub fn readlinkat(dirfd: i32, path: &str, buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_READLINKAT,
            in("x0") dirfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") buf.as_mut_ptr(),
            in("x4") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_209: Create a hard link.
#[inline]
pub fn linkat(olddfd: i32, oldpath: &str, newdfd: i32, newpath: &str, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_LINKAT,
            in("x0") olddfd,
            in("x1") oldpath.as_ptr(),
            in("x2") oldpath.len(),
            in("x3") newdfd,
            in("x4") newpath.as_ptr(),
            in("x5") newpath.len(),
            in("x6") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_198: UTIME_NOW - set to current time
pub const UTIME_NOW: u64 = 0x3FFFFFFF;
/// TEAM_198: UTIME_OMIT - don't change
pub const UTIME_OMIT: u64 = 0x3FFFFFFE;

/// TEAM_198: Set file access and modification times.
#[inline]
pub fn utimensat(dirfd: i32, path: &str, times: Option<&[Timespec; 2]>, flags: u32) -> isize {
    let ret: i64;
    let times_ptr = match times {
        Some(t) => t.as_ptr() as usize,
        None => 0,
    };
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_UTIMENSAT,
            in("x0") dirfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") times_ptr,
            in("x4") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_233: Create a pipe.
#[inline]
pub fn pipe2(pipefd: &mut [i32; 2], flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_PIPE2,
            in("x0") pipefd.as_mut_ptr(),
            in("x1") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_233: Duplicate a file descriptor to lowest available.
#[inline]
pub fn dup(oldfd: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_DUP,
            in("x0") oldfd,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_233: Duplicate a file descriptor to a specific number.
#[inline]
pub fn dup2(oldfd: usize, newfd: usize) -> isize {
    dup3(oldfd, newfd, 0)
}

/// TEAM_233: Duplicate a file descriptor with flags.
#[inline]
pub fn dup3(oldfd: usize, newfd: usize, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_DUP3,
            in("x0") oldfd,
            in("x1") newfd,
            in("x2") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}
