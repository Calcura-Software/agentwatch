// ── Detection types ──────────────────────────────────────────────────────────
// Core data types for the process classification pipeline.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The role a process plays within an agent ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ProcessRole {
    /// Primary user-facing session (e.g. `claude --`, `aider`, `gemini`)
    Main,
    /// Child worker spawned by a main agent (e.g. Claude `--input-format stream-json`)
    Subagent,
    /// Utility process that supports an agent but isn't an agent itself
    Helper,
    /// OS-level wrapper around an agent (e.g. `sandbox-exec`)
    Wrapper,
    /// IDE or app that hosts agent functionality (e.g. Cursor, VS Code)
    HostApp,
    /// Long-running server (e.g. MCP servers, language servers)
    Server,
    /// Could not determine role
    #[default]
    Unknown,
}


impl fmt::Display for ProcessRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessRole::Main     => write!(f, "main"),
            ProcessRole::Subagent => write!(f, "subagent"),
            ProcessRole::Helper   => write!(f, "helper"),
            ProcessRole::Wrapper  => write!(f, "wrapper"),
            ProcessRole::HostApp  => write!(f, "host_app"),
            ProcessRole::Server   => write!(f, "server"),
            ProcessRole::Unknown  => write!(f, "unknown"),
        }
    }
}

/// Static fingerprint definition for matching a process to an agent.
///
/// Each entry in the fingerprint database is one of these.  The `requires`
/// patterns use pipe (`|`) as OR within a single pattern, and ALL patterns
/// must match (logical AND across the slice).
#[derive(Clone)]
pub struct AgentFingerprint {
    pub id:         &'static str,
    pub name:       &'static str,
    pub icon:       &'static str,
    pub vendor:     &'static str,
    pub color:      &'static str,
    pub role:       ProcessRole,
    pub confidence: u8,
    pub requires:       &'static [&'static str],
    pub excludes:       &'static [&'static str],
    pub host_app:       Option<&'static str>,
    pub adopt_children: bool,
}

/// Result of classifying a single process against the fingerprint DB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub agent_id:     String,
    pub name:         String,
    pub icon:         String,
    pub vendor:       String,
    pub color:        String,
    pub role:         ProcessRole,
    pub host_app:     Option<String>,
    pub confidence:     u8,
    pub match_reason:   Vec<String>,
    pub adopt_children: bool,
}
