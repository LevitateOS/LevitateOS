//! TEAM_233: File descriptor duplication syscalls.

use crate::syscall::{errno, errno_file};
use crate::task::current_task;

/// TEAM_233: sys_dup - Duplicate a file descriptor to lowest available.
///
/// Returns new fd on success, negative errno on failure.
pub fn sys_dup(oldfd: usize) -> i64 {
    let task = current_task();
    let mut fd_table = task.fd_table.lock();

    match fd_table.dup(oldfd) {
        Some(newfd) => newfd as i64,
        None => errno::EBADF,
    }
}

/// TEAM_233: sys_dup3 - Duplicate a file descriptor to specific number.
///
/// If newfd is already open, it is closed first.
/// Returns newfd on success, negative errno on failure.
pub fn sys_dup3(oldfd: usize, newfd: usize, _flags: u32) -> i64 {
    if oldfd == newfd {
        return errno::EINVAL;
    }

    let task = current_task();
    let mut fd_table = task.fd_table.lock();

    match fd_table.dup_to(oldfd, newfd) {
        Some(fd) => fd as i64,
        None => errno::EBADF,
    }
}

/// TEAM_233: sys_pipe2 - Create a pipe.
///
/// Creates a pipe and returns two file descriptors in pipefd array.
/// pipefd[0] is the read end, pipefd[1] is the write end.
///
/// Returns 0 on success, negative errno on failure.
pub fn sys_pipe2(pipefd_ptr: usize, _flags: u32) -> i64 {
    use crate::fs::pipe::Pipe;
    use crate::memory::user as mm_user;
    use crate::task::fd_table::FdType;

    let task = current_task();

    // Validate user buffer (2 * sizeof(i32) = 8 bytes)
    if mm_user::validate_user_buffer(task.ttbr0, pipefd_ptr, 8, true).is_err() {
        return errno::EFAULT;
    }

    // Create the pipe
    let pipe = Pipe::new();

    // Allocate file descriptors
    let (read_fd, write_fd) = {
        let mut fd_table = task.fd_table.lock();

        let read_fd = match fd_table.alloc(FdType::PipeRead(pipe.clone())) {
            Some(fd) => fd,
            None => return errno_file::EMFILE,
        };

        let write_fd = match fd_table.alloc(FdType::PipeWrite(pipe.clone())) {
            Some(fd) => fd,
            None => {
                // Clean up read fd
                fd_table.close(read_fd);
                return errno_file::EMFILE;
            }
        };

        (read_fd, write_fd)
    };

    // Write fds to user space
    if let Some(ptr) = mm_user::user_va_to_kernel_ptr(task.ttbr0, pipefd_ptr) {
        unsafe {
            let fds = ptr as *mut [i32; 2];
            (*fds)[0] = read_fd as i32;
            (*fds)[1] = write_fd as i32;
        }
    } else {
        // Should not happen after validate_user_buffer, but handle it
        let mut fd_table = task.fd_table.lock();
        fd_table.close(read_fd);
        fd_table.close(write_fd);
        return errno::EFAULT;
    }

    log::trace!(
        "[SYSCALL] pipe2: created pipe fds [{}, {}]",
        read_fd,
        write_fd
    );
    0
}

/// TEAM_244: sys_isatty - Check if fd refers to a terminal.
///
/// Returns 1 if tty, 0 if not, negative errno on error.
pub fn sys_isatty(fd: i32) -> i64 {
    // In LevitateOS, stdin (0), stdout (1), stderr (2) are always TTYs
    // connected to the console
    match fd {
        0 | 1 | 2 => 1, // stdin, stdout, stderr are TTYs
        _ => {
            // Check if fd is valid
            let task = current_task();
            let fd_table = task.fd_table.lock();
            if fd_table.get(fd as usize).is_some() {
                0 // Valid fd but not a TTY
            } else {
                errno::EBADF // Invalid fd
            }
        }
    }
}
