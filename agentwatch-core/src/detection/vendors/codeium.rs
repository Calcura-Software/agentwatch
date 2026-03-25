// ── Codeium: Windsurf IDE ────────────────────────────────────────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    AgentFingerprint {
        id: "windsurf", name: "Windsurf", icon: "〜",
        vendor: "Codeium", color: "#06b6d4",
        role: ProcessRole::HostApp, confidence: 80,
        requires: &["windsurf|Windsurf|codeium"],
        excludes: &["grep"],
        host_app: Some("Windsurf"),
        adopt_children: false,
    },
];
