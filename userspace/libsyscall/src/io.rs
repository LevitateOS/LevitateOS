//! Core I/O
use crate::sched::sched_yield;
use crate::sysno::{SYS_CLOSE, SYS_IOCTL, SYS_READ, SYS_READV, SYS_WRITE, SYS_WRITEV};

/// TEAM_217: struct iovec for writev/readv
#[repr(C)]
pub struct IoVec {
    pub base: *const u8,
    pub len: usize,
}

/// TEAM_217: Vectored write.
#[inline]
pub fn writev(fd: usize, iov: &[IoVec]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_WRITEV,
            in("x0") fd,
            in("x1") iov.as_ptr(),
            in("x2") iov.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_217: Vectored read.
#[inline]
pub fn readv(fd: usize, iov: &mut [IoVec]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_READV,
            in("x0") fd,
            in("x1") iov.as_mut_ptr(),
            in("x2") iov.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// Read from a file descriptor.
#[inline]
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    loop {
        let ret: i64;
        unsafe {
            core::arch::asm!(
                "svc #0",
                in("x8") SYS_READ,
                in("x0") fd,
                in("x1") buf.as_mut_ptr(),
                in("x2") buf.len(),
                lateout("x0") ret,
                options(nostack)
            );
        }
        if ret == -11 {
            // EAGAIN
            sched_yield();
            continue;
        }
        return ret as isize;
    }
}

/// Write to a file descriptor.
#[inline]
pub fn write(fd: usize, buf: &[u8]) -> isize {
    loop {
        let ret: i64;
        unsafe {
            core::arch::asm!(
                "svc #0",
                in("x8") SYS_WRITE,
                in("x0") fd,
                in("x1") buf.as_ptr(),
                in("x2") buf.len(),
                lateout("x0") ret,
                options(nostack)
            );
        }
        if ret == -11 {
            // EAGAIN
            sched_yield();
            continue;
        }
        return ret as isize;
    }
}

/// Close a file descriptor.
#[inline]
pub fn close(fd: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_CLOSE,
            in("x0") fd,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_247: Generic ioctl wrapper.
#[inline]
pub fn ioctl(fd: usize, request: u64, arg: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_IOCTL,
            in("x0") fd,
            in("x1") request,
            in("x2") arg,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}
