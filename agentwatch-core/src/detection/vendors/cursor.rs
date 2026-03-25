// ── Cursor: Cursor IDE agent ─────────────────────────────────────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    AgentFingerprint {
        id: "cursor-agent", name: "Cursor Agent", icon: "⌘",
        vendor: "Cursor", color: "#a78bfa",
        role: ProcessRole::HostApp, confidence: 85,
        requires: &["Cursor|cursor-agent|cursor/resources"],
        excludes: &["grep", "AgentWatch", "CursorUIViewService", "TextInputUI"],
        host_app: Some("Cursor"),
        adopt_children: false,
    },
];
