// ── Microsoft: VS Code extension host ────────────────────────────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    AgentFingerprint {
        id: "vscode-agent", name: "VS Code Agent", icon: "⬡",
        vendor: "Microsoft", color: "#007acc",
        role: ProcessRole::HostApp, confidence: 75,
        requires: &["extensionHost|vscode-server|code-server"],
        excludes: &["grep"],
        host_app: Some("VS Code"),
        adopt_children: false,
    },
];
