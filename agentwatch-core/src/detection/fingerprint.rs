// ── Fingerprint database & classification ────────────────────────────────────
// Composes vendor-specific fingerprints into a single ordered database.
// Ordering matters: earlier entries take precedence (e.g. subagent before main).

use std::sync::OnceLock;
use super::types::{AgentFingerprint, DetectionResult};
use super::vendors;

/// All vendor fingerprint slices in precedence order.
/// Within each vendor, order is defined by the vendor module.
/// Across vendors, Anthropic and OpenAI come first because they have
/// precedence-sensitive entries (subagent before main, app before CLI).
const VENDOR_SLICES: &[&[AgentFingerprint]] = &[
    vendors::anthropic::FINGERPRINTS,  // claude-subagent before claude-code
    vendors::openai::FINGERPRINTS,     // codex-app before codex-cli
    vendors::google::FINGERPRINTS,
    vendors::cursor::FINGERPRINTS,
    vendors::microsoft::FINGERPRINTS,
    vendors::codeium::FINGERPRINTS,
    vendors::others::FINGERPRINTS,     // commander, opencode, continue, aider, mcp
];

/// Flattened fingerprint database, built once on first access.
/// This replaces the old `const AGENT_DB` to support vendor module composition.
fn agent_db() -> &'static [AgentFingerprint] {
    static DB: OnceLock<Vec<AgentFingerprint>> = OnceLock::new();
    DB.get_or_init(|| {
        VENDOR_SLICES.iter().flat_map(|s| s.iter().cloned()).collect()
    })
}

/// Public accessor for tests and external code that needs to inspect the DB.
pub fn get_agent_db() -> &'static [AgentFingerprint] {
    agent_db()
}

/// Check whether ALL required patterns match the command string.
/// Each pattern is pipe-delimited alternatives (OR), and all patterns
/// must match (AND across the slice).
pub fn matches_patterns(cmd: &str, patterns: &[&str]) -> bool {
    patterns.iter().all(|p| p.split('|').any(|sub| cmd.contains(sub)))
}

/// Classify a command string against the fingerprint DB.
/// Returns the first matching `DetectionResult`, or `None` if no match.
///
/// Order matters: earlier entries in the DB take precedence.
/// This is how `claude-subagent` wins over `claude-code` when both match.
pub fn classify_process(cmd: &str) -> Option<DetectionResult> {
    agent_db().iter().find(|fp| {
        matches_patterns(cmd, fp.requires)
            && !fp.excludes.iter().any(|e| cmd.contains(e))
    }).map(|fp| {
        // Record which substring actually matched for debugging
        let reasons: Vec<String> = fp.requires.iter()
            .filter_map(|p| {
                p.split('|')
                    .find(|sub| cmd.contains(sub))
                    .map(|s| format!("matched '{}'", s))
            })
            .collect();

        DetectionResult {
            agent_id:       fp.id.to_string(),
            name:           fp.name.to_string(),
            icon:           fp.icon.to_string(),
            vendor:         fp.vendor.to_string(),
            color:          fp.color.to_string(),
            role:           fp.role,
            host_app:       fp.host_app.map(|s| s.to_string()),
            confidence:     fp.confidence,
            match_reason:   reasons,
            adopt_children: fp.adopt_children,
        }
    })
}
