//! TEAM_171: Process management system calls.

use crate::syscall::errno;
use los_hal::println;

/// TEAM_073: sys_exit - Terminate the process.
pub fn sys_exit(code: i32) -> i64 {
    println!("[SYSCALL] exit({})", code);
    crate::task::task_exit();
}

/// TEAM_073: sys_getpid - Get process ID.
pub fn sys_getpid() -> i64 {
    crate::task::current_task().id.0 as i64
}

/// TEAM_129: sys_yield - Voluntarily yield CPU to other tasks.
pub fn sys_yield() -> i64 {
    crate::task::yield_now();
    0
}

/// TEAM_120: sys_spawn - Spawn a new process from initramfs.
pub fn sys_spawn(path_ptr: usize, path_len: usize) -> i64 {
    let path_len = path_len.min(256);
    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
        return errno::EFAULT;
    }

    let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
    let path = match core::str::from_utf8(path_bytes) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    println!("[SYSCALL] spawn('{}')", path);

    let archive_lock = crate::fs::INITRAMFS.lock();
    let archive = match archive_lock.as_ref() {
        Some(a) => a,
        None => return errno::ENOSYS,
    };

    let mut elf_data = None;
    for entry in archive.iter() {
        if entry.name == path {
            elf_data = Some(entry.data);
            break;
        }
    }

    let elf_data = match elf_data {
        Some(d) => d,
        None => return errno::EBADF,
    };

    match crate::task::process::spawn_from_elf(elf_data) {
        Ok(task) => {
            let pid = task.pid.0 as i64;
            crate::task::scheduler::SCHEDULER.add_task(alloc::sync::Arc::new(task.into()));
            pid
        }
        Err(e) => {
            println!("[SYSCALL] spawn failed: {:?}", e);
            -1
        }
    }
}

/// TEAM_120: sys_exec - Replace current process with one from initramfs.
pub fn sys_exec(path_ptr: usize, path_len: usize) -> i64 {
    let path_len = path_len.min(256);
    let task = crate::task::current_task();
    if crate::task::user_mm::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
        return errno::EFAULT;
    }

    let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
    let path = match core::str::from_utf8(path_bytes) {
        Ok(s) => s,
        Err(_) => return errno::EINVAL,
    };

    println!("[SYSCALL] exec('{}')", path);

    let archive_lock = crate::fs::INITRAMFS.lock();
    let archive = match archive_lock.as_ref() {
        Some(a) => a,
        None => return errno::ENOSYS,
    };

    let mut elf_data = None;
    for entry in archive.iter() {
        if entry.name == path {
            elf_data = Some(entry.data);
            break;
        }
    }

    let _elf_data = match elf_data {
        Some(d) => d,
        None => return errno::EBADF,
    };

    println!("[SYSCALL] exec is currently a stub");
    errno::ENOSYS
}
