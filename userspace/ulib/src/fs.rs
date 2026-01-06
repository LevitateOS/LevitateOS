//! TEAM_168: File system abstractions for LevitateOS userspace.
//!
//! Provides a `File` type similar to `std::fs::File`.

use crate::io::{Error, ErrorKind, Read, Result};

/// TEAM_168: An open file handle.
///
/// When dropped, the file is automatically closed.
///
/// # Example
/// ```rust
/// use ulib::fs::File;
///
/// let file = File::open("hello.txt")?;
/// let mut buf = [0u8; 100];
/// let n = file.read(&mut buf)?;
/// ```
pub struct File {
    fd: usize,
}

impl File {
    /// TEAM_168: Open a file for reading.
    ///
    /// # Arguments
    /// * `path` - Path to the file (in initramfs)
    ///
    /// # Returns
    /// The opened file, or an error.
    pub fn open(path: &str) -> Result<Self> {
        let fd = libsyscall::openat(path, 0); // 0 = read-only
        if fd < 0 {
            return Err(Error::from_errno(fd));
        }
        Ok(Self { fd: fd as usize })
    }

    /// TEAM_168: Get the file descriptor number.
    pub fn as_raw_fd(&self) -> usize {
        self.fd
    }

    /// TEAM_168: Get file metadata.
    pub fn metadata(&self) -> Result<Metadata> {
        let mut stat = libsyscall::Stat::default();
        let ret = libsyscall::fstat(self.fd, &mut stat);
        if ret < 0 {
            return Err(Error::from_errno(ret));
        }
        Ok(Metadata {
            size: stat.st_size,
            is_file: stat.st_mode == 1,
        })
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // TEAM_168: For initramfs files, we need to implement read properly
        // For now, this is a stub - full implementation requires kernel-side
        // read tracking per fd
        //
        // TODO(TEAM_168): Implement file read with position tracking
        Err(Error::new(ErrorKind::NotImplemented))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // Ignore errors on close - nothing we can do
        let _ = libsyscall::close(self.fd);
    }
}

/// TEAM_168: File metadata.
#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    /// File size in bytes
    pub size: u64,
    /// Whether this is a regular file
    pub is_file: bool,
}

impl Metadata {
    /// TEAM_168: Get the file size.
    pub fn len(&self) -> u64 {
        self.size
    }

    /// TEAM_168: Check if file is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// TEAM_168: Check if this is a regular file.
    pub fn is_file(&self) -> bool {
        self.is_file
    }
}
