// ── Other agents: Commander, OpenClaw, OpenCode, Antigravity, Continue, Aider, MCP

use crate::detection::types::{AgentFingerprint, ProcessRole};

pub const FINGERPRINTS: &[AgentFingerprint] = &[
    // Commander (autohandai) — Tauri app orchestrating claude/codex/gemini
    AgentFingerprint {
        id: "commander", name: "Commander", icon: "⌁",
        vendor: "Autohand AI", color: "#e879f9",
        role: ProcessRole::Main, confidence: 85,
        requires: &["commander|autohand|.commander/"],
        excludes: &["grep", "agentwatch"],
        host_app: None,
        adopt_children: true,
    },

    // OpenClaw (formerly Clawdbot → Moltbot → OpenClaw)
    // Self-hosted AI agent runtime + message router (Node.js daemon)
    // Config dir: ~/.moltbot (kept from Moltbot era)
    AgentFingerprint {
        id: "openclaw", name: "OpenClaw", icon: "✶",
        vendor: "OpenClaw", color: "#ef4444",
        role: ProcessRole::Main, confidence: 90,
        requires: &["openclaw|moltbot|clawdbot|.moltbot/"],
        excludes: &["grep", "agentwatch"],
        host_app: None,
        adopt_children: true,
    },

    // OpenCode
    AgentFingerprint {
        id: "opencode", name: "OpenCode", icon: "○",
        vendor: "OpenCode", color: "#38bdf8",
        role: ProcessRole::Main, confidence: 80,
        requires: &["opencode"],
        excludes: &["grep"],
        host_app: None,
        adopt_children: false,
    },

    // Antigravity
    AgentFingerprint {
        id: "antigravity", name: "Antigravity", icon: "∞",
        vendor: "Antigravity", color: "#818cf8",
        role: ProcessRole::Main, confidence: 80,
        requires: &["antigravity"],
        excludes: &["grep"],
        host_app: None,
        adopt_children: false,
    },

    // Continue.dev
    AgentFingerprint {
        id: "continue", name: "Continue", icon: "▷",
        vendor: "Continue", color: "#f59e0b",
        role: ProcessRole::Helper, confidence: 75,
        requires: &["continue|continuedev"],
        excludes: &["grep"],
        host_app: Some("VS Code / Cursor"),
        adopt_children: false,
    },

    // Aider
    AgentFingerprint {
        id: "aider", name: "Aider", icon: "⚡",
        vendor: "Aider", color: "#f97316",
        role: ProcessRole::Main, confidence: 90,
        requires: &["aider"],
        excludes: &["grep"],
        host_app: None,
        adopt_children: false,
    },

    // MCP servers
    AgentFingerprint {
        id: "mcp", name: "MCP Server", icon: "⬡",
        vendor: "Various", color: "#8b5cf6",
        role: ProcessRole::Server, confidence: 70,
        requires: &["mcp-server|sequential-thinking|playwright-mcp|github-mcp"],
        excludes: &["grep", "AgentWatch"],
        host_app: None,
        adopt_children: false,
    },
];
