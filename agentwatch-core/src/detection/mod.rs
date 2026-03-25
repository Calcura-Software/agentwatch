// ── Detection module ─────────────────────────────────────────────────────────
// Classifies OS processes into known AI agent types with role, confidence,
// and match reasoning.

pub mod types;
pub mod vendors;
pub mod fingerprint;
pub mod grouping;

#[cfg(test)]
mod tests;

// Re-exports for convenience
pub use types::{ProcessRole, DetectionResult, AgentFingerprint};
pub use fingerprint::{classify_process, get_agent_db, matches_patterns};
pub use grouping::generate_session_key;
