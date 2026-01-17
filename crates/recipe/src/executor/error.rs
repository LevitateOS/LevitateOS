//! Executor error types.

use thiserror::Error;

/// Errors that can occur during recipe execution.
#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("command failed: {cmd}\nstderr: {stderr}")]
    CommandFailedWithStderr { cmd: String, stderr: String },

    #[error("command failed: {cmd} (exit code: {code:?})")]
    CommandFailed { cmd: String, code: Option<i32> },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("no binary URL for architecture: {0}")]
    NoUrlForArch(String),

    #[error("sha256 verification failed: expected {expected}, got {actual}")]
    Sha256Mismatch { expected: String, actual: String },

    #[error("checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("unsupported archive format: {0}")]
    UnsupportedFormat(String),

    #[error("missing required field: {0}")]
    MissingField(String),

    #[error("feature resolution error: {0}")]
    FeatureError(String),
}
