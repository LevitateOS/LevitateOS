//! LevitateOS-specific components and philosophy.
//!
//! ## What Makes LevitateOS Different
//!
//! We borrow:
//! - Binaries and libraries from Fedora (fedora.rs)
//! - Reference implementation study from Arch (arch.rs)
//! - Fedora's release schedule (not rolling)
//!
//! We deliberately EXCLUDE:
//! - dnf, rpm (Fedora's package manager)
//! - pacman (Arch's package manager)
//! - Any traditional package manager
//!
//! Instead, we provide:
//! - AI-native software management
//! - FunctionGemma (offline SLM for OS tasks)
//! - OpenCode CLI (online AI agent)
//! - Rhai recipe system (AI generates, interpreter executes)
//! - Declarative system config (~/.levitate/system.rhai)
//!
//! ## Architecture
//!
//! ```text
//! User -> AI (online/offline) -> Rhai Recipe -> Build -> Install
//! ```

use anyhow::Result;
use std::path::Path;

/// LevitateOS tools built from source (in tools/ directory).
pub const LEVITATE_TOOLS: &[&str] = &[
    // Phase 1: Rhai recipe interpreter
    // "rhai-runner",

    // Phase 2: FunctionGemma LLM runner
    // "llm-runner",

    // Phase 4: System apply/diff tool
    // "levitate-cli",
];

/// External tools to bundle (downloaded to vendor/).
pub const EXTERNAL_TOOLS: &[(&str, &str)] = &[
    // Phase 3: OpenCode CLI
    // ("vendor/opencode/opencode", "bin/opencode"),
];

/// AI models to include in initramfs.
pub const MODELS: &[(&str, &str)] = &[
    // Phase 2: FunctionGemma
    // ("vendor/models/functiongemma-270m.gguf", "usr/lib/levitate/models/functiongemma.gguf"),
];

/// Copy LevitateOS-specific tools to initramfs.
///
/// Unlike fedora.rs (extracts Fedora binaries) or arch.rs (extracts Arch binaries),
/// this copies tools we build ourselves.
pub fn copy_tools(_root: &Path) -> Result<()> {
    // TODO: Implement in Phase 1+
    Ok(())
}

/// Copy AI models to initramfs.
pub fn copy_models(_root: &Path) -> Result<()> {
    // TODO: Implement in Phase 2
    Ok(())
}
