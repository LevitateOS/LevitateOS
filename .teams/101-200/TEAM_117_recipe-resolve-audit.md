# TEAM_117: Recipe Resolve - Phase 2 Security Audit

## Date: 2026-01-25

## Scope
Deep audit of recipe resolve implementation to find bugs, security issues, edge cases, and dead code.

---

## DEAD CODE ANALYSIS

### resolve_dep() Status
- **Location:** `leviso/src/resolve.rs`
- **Status:** Fully implemented, ZERO callsites in entire codebase
- **leviso-deps::DependencyResolver:** Actively used in 9 files:
  1. `src/main.rs:165` - Creates resolver on every run
  2. `src/commands/build.rs` - `resolver.rocky_iso()`, `resolver.linux()`
  3. `src/commands/download.rs` - All download targets
  4. `src/commands/extract.rs` - `resolver.rocky_iso()`
  5. `src/commands/show.rs` - `resolver.print_status()`
  6. `src/preflight/dependencies.rs` - Dependency checks
  7. `src/commands/clean.rs` - `resolver.clear_cache()`
  8. `src/component/custom/live.rs:104-106` - Separate resolver

### Decision (from TEAM_117)
Phase 6 (remove leviso-deps) was intentionally deferred. `leviso-deps` is proven reliable for production builds, while `recipe resolve` is newer infrastructure kept as alternative.

---

## SECURITY VULNERABILITIES

### CRITICAL

#### 1. Unbounded Download Size (DoS via Disk Exhaustion)
**File:** `tools/recipe/src/helpers/acquire.rs:86-105`

```rust
loop {
    let bytes_read = reader.read(&mut buffer)...;
    if bytes_read == 0 { break; }
    file.write_all(&buffer[..bytes_read])...;
    total_bytes += bytes_read as u64;
    // NO SIZE LIMIT CHECK
}
```

**Problem:** Malicious server can send infinite data, filling disk completely.

**Attack:** `download("http://attacker.com/infinite");` → system runs out of disk space

**Fix:**
```rust
const MAX_DOWNLOAD_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB
if total_bytes > MAX_DOWNLOAD_SIZE {
    return Err(format!("Download exceeded maximum size").into());
}
```

---

#### 2. Shell Command Injection in rpm_install()
**File:** `tools/recipe/src/helpers/install.rs:223-231`

```rust
let output = Command::new("sh")
    .args([
        "-c",
        &format!(
            "rpm2cpio '{}' | cpio -idmv -D '{}' 2>&1",
            rpm.display(),
            ctx.prefix.display()
        ),
    ])
```

**Problem:** Path values interpolated into shell command without escaping.

**Attack:** If path contains `$(malicious)` or backticks, code executes.

**Fix:** Use `shell_escape::unix::escape()`:
```rust
use shell_escape::unix::escape;
let escaped_rpm = escape(rpm.display().to_string().into());
let escaped_prefix = escape(ctx.prefix.display().to_string().into());
```

---

#### 3. Non-Atomic State Updates (State Corruption)
**File:** `tools/recipe/src/core/lifecycle.rs:332-368`

```rust
recipe_state::set_var(recipe_path, "installed", &true)?;      // Step 1
recipe_state::set_var(recipe_path, "installed_version", ...)?; // Step 2
recipe_state::set_var(recipe_path, "installed_at", ...)?;      // Step 3
recipe_state::set_var(recipe_path, "installed_files", ...)?;   // Step 4
```

**Problem:** If process crashes between steps, state is partially written. Package appears installed but file list is empty.

**Consequence:**
- Removal fails because file list is incomplete
- State cannot be recovered automatically

**Fix:** Write all state variables atomically via single temp file + rename.

---

#### 4. Version String Injection
**File:** `tools/recipe/src/core/lifecycle.rs:609`

```rust
if let Some(ver_str) = new_version.clone().try_cast::<String>() ... {
    recipe_state::set_var(&recipe_path_canonical, "version", &ver_str)?;
}
```

**Problem:** Version string from `check_update()` written directly without validation.

**Attack:** Malicious recipe returns `"; rm -rf /; //` as version, corrupting recipe file.

**Fix:**
```rust
fn validate_version(ver: &str) -> bool {
    ver.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}
```

---

#### 5. Incomplete Download Treated as Success
**File:** `tools/recipe/src/helpers/acquire.rs:105`

```rust
// No check that total_bytes == content_length
output::detail(&format!("downloaded {} ({} bytes)", filename, total_bytes));
ctx.last_downloaded = Some(dest);
Ok(())
```

**Problem:** If network cuts off mid-download, truncated file marked as success.

**Consequence:** ISO build fails with cryptic errors because ISO is incomplete.

**Fix:**
```rust
if let Some(expected) = content_length {
    if total_bytes != expected {
        return Err(format!("Download incomplete: got {} of {} bytes", total_bytes, expected).into());
    }
}
```

---

### HIGH SEVERITY

#### 6. TOCTOU Race in install_to_dir()
**File:** `tools/recipe/src/helpers/install.rs:143-155`

```rust
let dest_dir = ctx.prefix.join(subdir);
std::fs::create_dir_all(&dest_dir)?;  // Creates directory
validate_path_within_prefix(&dest_dir, &ctx.prefix)?;  // Validates AFTER creation
```

**Problem:** Between validation and copy, attacker can create symlink in `dest_dir`.

**Attack:**
1. Recipe calls `install_to_dir("malicious.conf", "share/doc")`
2. Attacker creates symlink: `/prefix/share/doc` -> `/etc`
3. File copied to `/etc/malicious.conf`

**Fix:** Validate after mkdir, use `O_NOFOLLOW` when opening directories.

---

#### 7. Git Clone Incomplete But Appears Valid
**File:** `tools/recipe/src/helpers/git.rs:72-79`

```rust
let verify = Command::new("git")
    .args(["-C", &dest.to_string_lossy(), "rev-parse", "HEAD"])
    ...
if verify.map(|s| s.success()).unwrap_or(false) {
    return Ok(dest.to_string_lossy().to_string());  // Skips re-clone
}
```

**Problem:** `git rev-parse HEAD` succeeds even if objects are missing from interrupted clone.

**Consequence:** Build proceeds with incomplete kernel source, cryptic errors later.

**Fix:** Run `git fsck --quick` after rev-parse to verify object integrity.

---

#### 8. Comment Parsing Broken in recipe_state.rs
**File:** `tools/recipe/src/core/recipe_state.rs:17-61`

**Problem:** Inline comments not properly stripped.

```rhai
let version = "1.0"; // this is a comment
```

Parses as: `"1.0"; // this is a comment` instead of `"1.0"`

**Consequence:** Recipes with inline comments on state variables fail to parse.

**Fix:** Strip `//` comments from end of lines before parsing value.

---

#### 9. resolve() Path Traversal Incomplete
**File:** `tools/recipe/src/core/lifecycle.rs:703-706`

```rust
let path = if path.is_relative() {
    build_dir.join(&path)  // No validation of ../ traversal
} else {
    path
};
let path = path.canonicalize()?;  // Canonicalizes but doesn't validate prefix
```

**Problem:** Recipe returns `../../../usr/bin/ls`, joins with build_dir, canonicalizes to `/usr/bin/ls`. No check that result is within expected directory.

**Fix:** After canonicalize, verify path starts with expected prefix:
```rust
if !path.starts_with(build_dir) {
    return Err(anyhow!("Resolved path escapes build directory"));
}
```

---

### MEDIUM SEVERITY

#### 10. HTTP URLs Accepted (MITM Risk)
**Files:** `git.rs:36-51`, `torrent.rs:33-44`

Both accept `http://` URLs for git clone and downloads.

**Attack:** Man-in-the-middle intercepts and replaces repository/files.

**Fix:** Remove `http://` support, require HTTPS only:
```rust
if url.starts_with("https://") || url.starts_with("git@") || url.starts_with("ssh://") {
    Ok(())
} else {
    Err("Only https://, ssh://, and git@ URLs are supported".into())
}
```

---

#### 11. Magnet Link Filename Collisions
**File:** `tools/recipe/src/helpers/torrent.rs:269-273`

```rust
if url.starts_with("magnet:") {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    return format!("magnet_{}", timestamp);
}
```

**Problem:** Two magnet downloads within same millisecond get same filename.

**Fix:** Add random suffix or use info_hash from magnet link:
```rust
format!("magnet_{}_{}", timestamp, rand::random::<u32>())
```

---

#### 12. Temp File Non-Unique Identifier
**File:** `tools/recipe/src/core/recipe_state.rs:129-135`

```rust
let temp_path = parent.join(format!(
    ".{}.tmp.{}",
    recipe_path.file_name()...,
    std::process::id()  // Same PID can run multiple times
));
```

**Problem:** PID reuses across time. Stale temp files from crashed processes accumulate.

**Fix:** Add timestamp or random component:
```rust
format!(".{}.tmp.{}.{}", filename, std::process::id(), std::time::SystemTime::now().elapsed()...)
```

---

#### 13. remove() State Set Before post_remove Hook
**File:** `tools/recipe/src/core/lifecycle.rs:500-525`

```rust
recipe_state::set_var(&recipe_path_canonical, "installed", &false)?;
// ...
if has_action(&ast, "post_remove") {
    let _ = call_action(engine, &mut scope, &ast, "post_remove");  // Error ignored!
}
```

**Problem:** If `post_remove` hook fails, package appears removed but cleanup incomplete.

**Fix:** Move state update after successful hook completion, don't ignore hook errors.

---

#### 14. Unsafe set_env() Thread Safety
**File:** `tools/recipe/src/helpers/env.rs:9-13`

```rust
pub fn set_env(name: &str, value: &str) {
    // SAFETY: We are setting env vars in a single-threaded recipe context.
    unsafe { std::env::set_var(name, value) };
}
```

**Problem:** Comment says single-threaded but not enforced. If recipes run in threads, this is data race.

**Fix:** Document requirement more clearly, or use thread-safe env var storage.

---

## INTENTIONAL DESIGN CHOICES (Not Bugs)

### Shell Command Execution in build.rs
**File:** `tools/recipe/src/helpers/build.rs:154-159`

Recipe scripts can pass arbitrary shell commands via `run()`. This is intentional - recipes are code.

**Recommendation:** Document that:
- Recipes must be reviewed before execution
- Run recipes in restricted environment/container for untrusted sources
- Never run untrusted recipe files directly

---

## POSITIVE FINDINGS (Secure)

These implementations are already secure:
- **acquire.rs hash verification:** Proper comparison using `==` on hex strings
- **install.rs path validation:** `validate_path_within_prefix()` canonicalizes paths
- **filesystem.rs glob:** Safe use of glob library
- **process.rs exec():** Safe `Command::new()` with array args (no shell)
- **git.rs URL validation:** Already added in Phase 1 (but needs HTTP removal)
- **torrent.rs URL validation:** Already added in Phase 1 (but needs HTTP removal)

---

## IMPLEMENTATION PRIORITY

### Immediate (This Sprint)
1. A.1 Unbounded download → Add MAX_DOWNLOAD_SIZE
2. A.2 Shell injection → Add shell_escape
3. B.1 Non-atomic state → Single atomic write
4. B.2 Version injection → Validate format
5. B.3 Incomplete download → Check Content-Length

### Urgent (Next Sprint)
1. C.1 Git incomplete → Add `git fsck --quick`
2. C.2 Comment parsing → Fix inline comment stripping
3. C.4 Path traversal → Validate canonical path prefix
4. A.3 TOCTOU → Validate after mkdir

### Important (Backlog)
1. A.4 HTTP URLs → Remove http:// support
2. C.3 Filename collision → Add random suffix
3. C.5 Temp file race → Add timestamp
4. C.6 Remove state race → Move state update

---

## FILES AFFECTED

| File | Issues |
|------|--------|
| `tools/recipe/src/helpers/acquire.rs` | A.1, B.3 |
| `tools/recipe/src/helpers/install.rs` | A.2, A.3 |
| `tools/recipe/src/helpers/git.rs` | A.4, C.1 |
| `tools/recipe/src/helpers/torrent.rs` | A.4, C.3 |
| `tools/recipe/src/core/lifecycle.rs` | B.1, B.2, C.4, C.6 |
| `tools/recipe/src/core/recipe_state.rs` | C.2, C.5 |
| `tools/recipe/src/helpers/env.rs` | #14 |
| `tools/recipe/Cargo.toml` | Add shell-escape |

---

## VERIFICATION TESTS TO ADD

```rust
// Security tests
#[test] fn test_download_size_limit() { ... }
#[test] fn test_version_validation_rejects_injection() { ... }
#[test] fn test_incomplete_download_detected() { ... }
#[test] fn test_http_urls_rejected() { ... }
#[test] fn test_resolve_path_traversal_blocked() { ... }

// State tests
#[test] fn test_state_atomic_on_crash() { ... }
#[test] fn test_inline_comments_stripped() { ... }

// Edge cases
#[test] fn test_git_clone_incomplete_detected() { ... }
#[test] fn test_magnet_filename_unique() { ... }
```
