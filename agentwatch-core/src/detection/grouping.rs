// ── Session grouping ─────────────────────────────────────────────────────────
// Generates session keys for clustering related processes on the backend.
// Replaces the React-side HOST_TO_AGENT fallback with a vendor-agnostic
// session linker (Phase 5 of the pair-review plan).

/// Map of host-app display names → primary agent IDs they host.
/// Used to normalize session keys so subagents/helpers share the same key
/// as their parent main session.
fn resolve_parent_agent<'a>(agent_id: &'a str, host_app: Option<&str>) -> &'a str {
    match host_app {
        Some("Claude Code")      => "claude-code",
        Some("Codex App")        => "codex-app",
        Some("Cursor")           => "cursor-agent",
        Some("VS Code")          => "vscode-agent",
        Some("VS Code / Cursor") => "vscode-agent",
        Some("Windsurf")         => "windsurf",
        _                        => agent_id,
    }
}

/// Generate a session key for a process.
///
/// Processes with a `host_app` are normalized to their parent agent's ID
/// so they cluster together. E.g. a claude-subagent working in `/proj`
/// gets key `"claude-code:/proj"`, matching its parent main session.
///
/// Falls back to `agent_id:pid:<parent_pid>` when cwd is unavailable.
pub fn generate_session_key(
    agent_id:   &str,
    cwd:        &str,
    parent_pid: Option<u32>,
    host_app:   Option<&str>,
) -> String {
    let resolved = resolve_parent_agent(agent_id, host_app);
    if cwd.is_empty() {
        format!("{}:pid:{}", resolved, parent_pid.unwrap_or(0))
    } else {
        format!("{}:{}", resolved, cwd)
    }
}
