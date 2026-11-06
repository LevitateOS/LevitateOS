//! Errors that pick xtask's exit code: 3 for a missing prerequisite, 2 for a
//! command that cannot work in this context, 1 for everything else.

use std::fmt;

/// Something must exist first; `fix` is the command that provides it.
#[derive(Debug)]
pub struct MissingPrereq {
    pub what: String,
    pub fix: String,
}

impl fmt::Display for MissingPrereq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n  fix: {}", self.what, self.fix)
    }
}

impl std::error::Error for MissingPrereq {}

pub fn missing(what: impl Into<String>, fix: impl Into<String>) -> anyhow::Error {
    MissingPrereq {
        what: what.into(),
        fix: fix.into(),
    }
    .into()
}

/// The arguments cannot work here; the message says what does.
#[derive(Debug)]
pub struct Usage(pub String);

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Usage {}

pub fn usage(message: impl Into<String>) -> anyhow::Error {
    Usage(message.into()).into()
}

pub fn code_for(error: &anyhow::Error) -> i32 {
    if error.chain().any(|e| e.is::<MissingPrereq>()) {
        3
    } else if error.chain().any(|e| e.is::<Usage>()) {
        2
    } else {
        1
    }
}

/// Fail (exit 2) unless stdin and stdout are a terminal, naming what works
/// without one.
pub fn require_terminal(what: &str, alternative: &str) -> anyhow::Result<()> {
    // SAFETY: isatty only inspects the given descriptors.
    let tty = unsafe { libc::isatty(0) == 1 && libc::isatty(1) == 1 };
    if tty {
        return Ok(());
    }
    Err(usage(format!(
        "{what} needs a terminal, and this one has none.\n  instead: {alternative}"
    )))
}
