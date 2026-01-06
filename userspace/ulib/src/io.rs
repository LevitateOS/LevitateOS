//! TEAM_168: I/O abstractions for LevitateOS userspace.
//!
//! Provides error types, traits, and common I/O functionality.

use core::fmt;

/// TEAM_168: Error codes from syscalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorKind {
    /// Function not implemented
    NotImplemented = -1,
    /// Bad file descriptor
    BadFd = -2,
    /// Bad address
    BadAddress = -3,
    /// Invalid argument
    InvalidArgument = -4,
    /// No such file or directory
    NotFound = -5,
    /// Too many open files
    TooManyFiles = -6,
    /// Unknown error
    Unknown = -99,
}

impl ErrorKind {
    /// TEAM_168: Convert from syscall return value.
    pub fn from_errno(errno: isize) -> Self {
        match errno {
            -1 => Self::NotImplemented,
            -2 => Self::BadFd,
            -3 => Self::BadAddress,
            -4 => Self::InvalidArgument,
            -5 => Self::NotFound,
            -6 => Self::TooManyFiles,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "function not implemented"),
            Self::BadFd => write!(f, "bad file descriptor"),
            Self::BadAddress => write!(f, "bad address"),
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::NotFound => write!(f, "no such file or directory"),
            Self::TooManyFiles => write!(f, "too many open files"),
            Self::Unknown => write!(f, "unknown error"),
        }
    }
}

/// TEAM_168: I/O Error type.
#[derive(Debug, Clone, Copy)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// TEAM_168: Create a new error from an error kind.
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    /// TEAM_168: Create an error from a syscall return value.
    pub fn from_errno(errno: isize) -> Self {
        Self::new(ErrorKind::from_errno(errno))
    }

    /// TEAM_168: Get the error kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// TEAM_168: Result type for I/O operations.
pub type Result<T> = core::result::Result<T, Error>;

/// TEAM_168: Read trait for types that can be read from.
pub trait Read {
    /// Read bytes into a buffer.
    ///
    /// Returns the number of bytes read, or an error.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Read exactly `buf.len()` bytes.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        let mut offset = 0;
        while offset < buf.len() {
            let n = self.read(&mut buf[offset..])?;
            if n == 0 {
                return Err(Error::new(ErrorKind::Unknown)); // EOF
            }
            offset += n;
        }
        Ok(())
    }
}

/// TEAM_168: Write trait for types that can be written to.
pub trait Write {
    /// Write bytes from a buffer.
    ///
    /// Returns the number of bytes written, or an error.
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Flush any buffered data.
    fn flush(&mut self) -> Result<()>;

    /// Write all bytes from a buffer.
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut offset = 0;
        while offset < buf.len() {
            let n = self.write(&buf[offset..])?;
            if n == 0 {
                return Err(Error::new(ErrorKind::Unknown));
            }
            offset += n;
        }
        Ok(())
    }
}
