// ── OpenAI: Codex App, Codex CLI, Codex Sandbox ─────────────────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    // Codex Native App / ChatGPT extension — must come BEFORE codex-cli.
    // Matches Codex.app and `codex app-server` (ChatGPT VS Code extension).
    AgentFingerprint {
        id: "codex-app", name: "Codex App", icon: "◈",
        vendor: "OpenAI", color: "#10a37f",
        role: ProcessRole::HostApp, confidence: 90,
        requires: &["Codex.app|codex-app-server|CodexAgent|codex app-server"],
        excludes: &["grep", "agentwatch"],
        host_app: Some("Codex App"),
        adopt_children: false,
    },

    // Codex CLI
    AgentFingerprint {
        id: "codex-cli", name: "Codex CLI", icon: "◈",
        vendor: "OpenAI", color: "#74aa9c",
        role: ProcessRole::Main, confidence: 95,
        requires: &["@openai/codex|codex-cli"],
        excludes: &["Codex.app", "codex app", "grep", "agentwatch"],
        host_app: None,
        adopt_children: true,
    },

    // Codex sandbox-exec (macOS Seatbelt wrapper)
    AgentFingerprint {
        id: "codex-sandbox", name: "Codex Sandboxed Agent", icon: "◈",
        vendor: "OpenAI", color: "#0d8a6a",
        role: ProcessRole::Wrapper, confidence: 85,
        requires: &["sandbox-exec", "codex"],
        excludes: &["grep", "agentwatch"],
        host_app: Some("Codex App"),
        adopt_children: false,
    },
];
