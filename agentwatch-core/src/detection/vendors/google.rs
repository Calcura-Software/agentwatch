// ── Google: Gemini CLI ───────────────────────────────────────────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    AgentFingerprint {
        id: "gemini", name: "Gemini CLI", icon: "✦",
        vendor: "Google", color: "#4285f4",
        role: ProcessRole::Main, confidence: 95,
        requires: &["@google/gemini-cli|/gemini"],
        excludes: &["grep", "agentwatch"],
        host_app: None,
        adopt_children: true,
    },
];
