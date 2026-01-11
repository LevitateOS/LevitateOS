//! Static libc for LevitateOS
//!
//! This crate builds c-gull as a static library (libc.a) that can be used
//! to link Rust programs without depending on glibc/musl.
//!
//! The resulting library provides:
//! - All standard libc functions (from c-gull/c-scape)
//! - Program startup code (from origin)
//! - malloc/free (from rustix-dlmalloc)
//! - Thread support
//! - TEAM_435: Stubs for missing user/group functions

#![no_std]

// Pull in c-gull which provides all libc symbols via #[no_mangle] exports
extern crate c_gull;

// Re-export everything to ensure the linker sees all symbols
pub use c_gull::*;

// =============================================================================
// TEAM_435: Missing libc function stubs
// c-gull doesn't provide these yet - LevitateOS has no users so return "not found"
// =============================================================================

use core::ffi::{c_char, c_int};

const ENOENT: c_int = 2;

// passwd structure (must match c-gull's definition)
#[repr(C)]
pub struct Passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

// group structure
#[repr(C)]
pub struct Group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: u32,
    pub gr_mem: *mut *mut c_char,
}

/// getpwnam_r - get password entry by name (reentrant)
/// Returns ENOENT since LevitateOS has no users
#[no_mangle]
pub unsafe extern "C" fn getpwnam_r(
    _name: *const c_char,
    _pwd: *mut Passwd,
    _buf: *mut c_char,
    _buflen: usize,
    result: *mut *mut Passwd,
) -> c_int {
    if !result.is_null() {
        *result = core::ptr::null_mut();
    }
    ENOENT
}

/// getgrgid_r - get group entry by GID (reentrant)
/// Returns ENOENT since LevitateOS has no groups
#[no_mangle]
pub unsafe extern "C" fn getgrgid_r(
    _gid: u32,
    _grp: *mut Group,
    _buf: *mut c_char,
    _buflen: usize,
    result: *mut *mut Group,
) -> c_int {
    if !result.is_null() {
        *result = core::ptr::null_mut();
    }
    ENOENT
}

/// getgrouplist - get list of groups for user
/// Returns 1 group (the primary gid) since LevitateOS has no supplementary groups
#[no_mangle]
pub unsafe extern "C" fn getgrouplist(
    _user: *const c_char,
    gid: u32,
    groups: *mut u32,
    ngroups: *mut c_int,
) -> c_int {
    if groups.is_null() || ngroups.is_null() {
        return -1;
    }
    let n = *ngroups;
    if n < 1 {
        *ngroups = 1;
        return -1;
    }
    *groups = gid;
    *ngroups = 1;
    1
}

/// ttyname_r - get terminal name (reentrant)
/// Returns ENOENT since we don't track tty names
#[no_mangle]
pub unsafe extern "C" fn ttyname_r(
    _fd: c_int,
    _buf: *mut c_char,
    _buflen: usize,
) -> c_int {
    ENOENT
}

// =============================================================================
// TEAM_438: posix_spawn_file_actions_addchdir (POSIX version)
// c-gull provides _np (non-portable/BSD) variant, this is the standard POSIX name
// =============================================================================

// Opaque type - actual struct is in c-gull
#[repr(C)]
pub struct PosixSpawnFileActions {
    _opaque: [u8; 0],
}

extern "C" {
    fn posix_spawn_file_actions_addchdir_np(
        actions: *mut PosixSpawnFileActions,
        path: *const c_char,
    ) -> c_int;
}

/// posix_spawn_file_actions_addchdir - POSIX standard version
/// Forwards to the _np (non-portable) version provided by c-gull
#[no_mangle]
pub unsafe extern "C" fn posix_spawn_file_actions_addchdir(
    actions: *mut PosixSpawnFileActions,
    path: *const c_char,
) -> c_int {
    posix_spawn_file_actions_addchdir_np(actions, path)
}
