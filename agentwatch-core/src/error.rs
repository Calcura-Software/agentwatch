//! Error types for AgentWatch Core
//!
//! Provides structured errors for agent detection and system operations.

use thiserror::Error;

/// Main error type for AgentWatch operations
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to scan system processes
    #[error("failed to scan processes: {reason}")]
    ScanFailed { reason: String },

    /// A specific process could not be accessed
    #[error("process {pid} not accessible: {reason}")]
    ProcessAccess { pid: u32, reason: String },

    /// System information unavailable
    #[error("system info unavailable: {0}")]
    SystemInfo(String),

    /// IO operation failed
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for AgentWatch operations
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ScanFailed {
            reason: "permission denied".to_string(),
        };
        assert_eq!(err.to_string(), "failed to scan processes: permission denied");
    }

    #[test]
    fn test_process_access_error() {
        let err = Error::ProcessAccess {
            pid: 1234,
            reason: "zombie process".to_string(),
        };
        assert!(err.to_string().contains("1234"));
        assert!(err.to_string().contains("zombie"));
    }
}
