// ── Anthropic: Claude Code, Claude Subagent, Claude Desktop ──────────────────

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    // Claude Code subagent workers — must come BEFORE claude-code.
    // These are JSON-API child processes spawned by the Agent tool.
    AgentFingerprint {
        id: "claude-subagent", name: "Claude Subagent", icon: "◇",
        vendor: "Anthropic", color: "#d4a088",
        role: ProcessRole::Subagent, confidence: 90,
        requires: &["claude", "--input-format stream-json"],
        excludes: &["agentwatch", "grep"],
        host_app: Some("Claude Code"),
        adopt_children: false,
    },

    // Claude Code main sessions: `claude --dangerously-skip-permissions` etc.
    // "/bin/claude" or "claude --" avoids matching Claude.app, MCP paths.
    AgentFingerprint {
        id: "claude-code", name: "Claude Code", icon: "◆",
        vendor: "Anthropic", color: "#cc785c",
        role: ProcessRole::Main, confidence: 95,
        requires: &["/bin/claude|claude --"],
        excludes: &["agentwatch", "grep", "ps ax", "--input-format stream-json", "Claude.app"],
        host_app: None,
        adopt_children: true,
    },

    // Claude Desktop App
    AgentFingerprint {
        id: "claude-desktop", name: "Claude Desktop", icon: "◆",
        vendor: "Anthropic", color: "#d97756",
        role: ProcessRole::Main, confidence: 90,
        requires: &["Claude.app|Claude Helper"],
        excludes: &["grep", "agentwatch"],
        host_app: None,
        adopt_children: false,
    },
];
