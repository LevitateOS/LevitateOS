//! TEAM_201: POSIX File Mode Constants
//!
//! TEAM_419: Re-exported from linux-raw-sys for authoritative Linux ABI values.

// TEAM_419: File mode constants from linux-raw-sys
pub use linux_raw_sys::general::{
    // File type constants
    S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR, S_IFIFO,
    // Permission constants
    S_ISUID, S_ISGID, S_ISVTX,
    S_IRWXU, S_IRUSR, S_IWUSR, S_IXUSR,
    S_IRWXG, S_IRGRP, S_IWGRP, S_IXGRP,
    S_IRWXO, S_IROTH, S_IWOTH, S_IXOTH,
};

// ============================================================================
// Helper Functions
// ============================================================================

/// TEAM_201: Check if mode indicates a regular file
#[inline]
pub const fn is_reg(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFREG
}

/// TEAM_201: Check if mode indicates a directory
#[inline]
pub const fn is_dir(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFDIR
}

/// TEAM_201: Check if mode indicates a symbolic link
#[inline]
pub const fn is_lnk(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFLNK
}

/// TEAM_201: Check if mode indicates a character device
#[inline]
pub const fn is_chr(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFCHR
}

/// TEAM_201: Check if mode indicates a block device
#[inline]
pub const fn is_blk(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFBLK
}

/// TEAM_201: Check if mode indicates a FIFO
#[inline]
pub const fn is_fifo(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFIFO
}

/// TEAM_201: Check if mode indicates a socket
#[inline]
pub const fn is_sock(mode: u32) -> bool {
    (mode & S_IFMT) == S_IFSOCK
}

/// TEAM_201: Extract just the file type from mode
#[inline]
pub const fn file_type(mode: u32) -> u32 {
    mode & S_IFMT
}

/// TEAM_201: Extract just the permission bits from mode
#[inline]
pub const fn permissions(mode: u32) -> u32 {
    mode & 0o7777
}

/// TEAM_201: Create a mode with file type and permissions
#[inline]
pub const fn make_mode(file_type: u32, perms: u32) -> u32 {
    (file_type & S_IFMT) | (perms & 0o7777)
}
