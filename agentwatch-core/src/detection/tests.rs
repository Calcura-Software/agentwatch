// ── Detection unit tests ─────────────────────────────────────────────────────
// Fixture-backed tests for fingerprint matching, precedence, exclusions,
// role assignment, and session grouping.

use super::fingerprint::{classify_process, matches_patterns, get_agent_db};
use super::grouping::generate_session_key;
use super::types::ProcessRole;

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// A test fixture: a fake command line + expected classification result.
struct Fixture {
    label:    &'static str,
    cmd:      &'static str,
    agent_id: Option<&'static str>,
    role:     Option<ProcessRole>,
}

fn assert_fixture(f: &Fixture) {
    let result = classify_process(f.cmd);
    match f.agent_id {
        Some(expected_id) => {
            let r = result.unwrap_or_else(|| {
                panic!("[{}] expected agent_id='{}' but got None\ncmd: {}", f.label, expected_id, f.cmd)
            });
            assert_eq!(r.agent_id, expected_id,
                "[{}] wrong agent_id — got '{}', expected '{}'\ncmd: {}",
                f.label, r.agent_id, expected_id, f.cmd);
            if let Some(expected_role) = f.role {
                assert_eq!(r.role, expected_role,
                    "[{}] wrong role — got '{}', expected '{}'\ncmd: {}",
                    f.label, r.role, expected_role, f.cmd);
            }
        }
        None => {
            assert!(result.is_none(),
                "[{}] expected NO match but got agent_id='{}'\ncmd: {}",
                f.label, result.as_ref().unwrap().agent_id, f.cmd);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Positive-match tests — one per agent
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_claude_subagent() {
    assert_fixture(&Fixture {
        label:    "claude-subagent via stream-json",
        cmd:      "/Users/user/.claude/local/node_modules/.bin/claude --input-format stream-json --output-format stream-json --verbose",
        agent_id: Some("claude-subagent"),
        role:     Some(ProcessRole::Subagent),
    });
}

#[test]
fn test_claude_code_bin() {
    assert_fixture(&Fixture {
        label:    "claude-code via /bin/claude",
        cmd:      "/usr/local/bin/claude --dangerously-skip-permissions",
        agent_id: Some("claude-code"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_claude_code_flag() {
    assert_fixture(&Fixture {
        label:    "claude-code via 'claude --'",
        cmd:      "claude -- fix the bug in auth.ts",
        agent_id: Some("claude-code"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_claude_desktop() {
    assert_fixture(&Fixture {
        label:    "Claude Desktop app",
        cmd:      "/Applications/Claude.app/Contents/MacOS/Claude Helper (Renderer) --type=renderer",
        agent_id: Some("claude-desktop"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_codex_app_native() {
    assert_fixture(&Fixture {
        label:    "Codex native app",
        cmd:      "/Applications/Codex.app/Contents/MacOS/Codex --type=renderer",
        agent_id: Some("codex-app"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_codex_app_server() {
    assert_fixture(&Fixture {
        label:    "codex app-server (ChatGPT extension)",
        cmd:      "node /Users/user/.vscode/extensions/openai.codex-0.1.0/dist/codex-app-server --port 12345",
        agent_id: Some("codex-app"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_codex_cli() {
    assert_fixture(&Fixture {
        label:    "Codex CLI via npm",
        cmd:      "node /Users/user/node_modules/@openai/codex/dist/cli.js --model gpt-4",
        agent_id: Some("codex-cli"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_codex_sandbox() {
    assert_fixture(&Fixture {
        label:    "Codex sandbox-exec wrapper",
        cmd:      "sandbox-exec -p (allow default) /usr/local/bin/codex --task build",
        agent_id: Some("codex-sandbox"),
        role:     Some(ProcessRole::Wrapper),
    });
}

#[test]
fn test_commander() {
    assert_fixture(&Fixture {
        label:    "Commander app",
        cmd:      "/Applications/Commander.app/Contents/MacOS/commander --project myapp",
        agent_id: Some("commander"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_commander_worktree() {
    assert_fixture(&Fixture {
        label:    "Commander worktree path",
        cmd:      "node /Users/user/projects/myapp/.commander/session-abc/worker.js",
        agent_id: Some("commander"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_gemini_cli() {
    assert_fixture(&Fixture {
        label:    "Gemini CLI via npm",
        cmd:      "node /Users/user/.npm/_npx/abc123/node_modules/@google/gemini-cli/dist/index.js",
        agent_id: Some("gemini"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_gemini_direct() {
    assert_fixture(&Fixture {
        label:    "Gemini direct binary",
        cmd:      "/usr/local/bin/gemini --model gemini-pro",
        agent_id: Some("gemini"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_openclaw_cli() {
    assert_fixture(&Fixture {
        label:    "OpenClaw CLI (global install)",
        cmd:      "node /usr/local/bin/openclaw gateway --port 18789 --verbose",
        agent_id: Some("openclaw"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_openclaw_moltbot_path() {
    assert_fixture(&Fixture {
        label:    "OpenClaw via .moltbot config dir",
        cmd:      "node /Users/user/.moltbot/daemon/server.js --port 18789",
        agent_id: Some("openclaw"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_openclaw_agent() {
    assert_fixture(&Fixture {
        label:    "OpenClaw agent command",
        cmd:      "openclaw agent --message hello --thinking high",
        agent_id: Some("openclaw"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_openclaw_legacy_clawdbot() {
    assert_fixture(&Fixture {
        label:    "Legacy Clawdbot name",
        cmd:      "node /usr/local/lib/node_modules/clawdbot/dist/index.js serve",
        agent_id: Some("openclaw"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_opencode() {
    assert_fixture(&Fixture {
        label:    "OpenCode",
        cmd:      "/usr/local/bin/opencode --config ~/.opencode.yaml",
        agent_id: Some("opencode"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_antigravity() {
    assert_fixture(&Fixture {
        label:    "Antigravity IDE",
        cmd:      "/Applications/Antigravity.app/Contents/MacOS/antigravity --new-window",
        agent_id: Some("antigravity"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_cursor_agent() {
    assert_fixture(&Fixture {
        label:    "Cursor IDE",
        cmd:      "/Applications/Cursor.app/Contents/MacOS/Cursor --type=renderer --enable-features=SharedArrayBuffer",
        agent_id: Some("cursor-agent"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_vscode_extension_host() {
    assert_fixture(&Fixture {
        label:    "VS Code extension host",
        cmd:      "/Applications/Visual Studio Code.app/Contents/Frameworks/Code Helper.app/Contents/MacOS/Code Helper --type=extensionHost --inspect=9229",
        agent_id: Some("vscode-agent"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_continue_dev() {
    assert_fixture(&Fixture {
        label:    "Continue.dev extension",
        cmd:      "node /Users/user/.vscode/extensions/continue.continue-0.9.0/out/extension.js",
        agent_id: Some("continue"),
        role:     Some(ProcessRole::Helper),
    });
}

#[test]
fn test_windsurf() {
    assert_fixture(&Fixture {
        label:    "Windsurf IDE",
        cmd:      "/Applications/Windsurf.app/Contents/MacOS/Windsurf --new-window",
        agent_id: Some("windsurf"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_windsurf_codeium() {
    assert_fixture(&Fixture {
        label:    "Codeium process (Windsurf engine)",
        cmd:      "node /Users/user/.codeium/bin/codeium_language_server --port 42100",
        agent_id: Some("windsurf"),
        role:     Some(ProcessRole::HostApp),
    });
}

#[test]
fn test_aider() {
    assert_fixture(&Fixture {
        label:    "Aider CLI",
        cmd:      "python3 -m aider --model claude-3.5-sonnet --auto-commits",
        agent_id: Some("aider"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_mcp_server_filesystem() {
    assert_fixture(&Fixture {
        label:    "MCP server (filesystem)",
        cmd:      "node /Users/user/.claude/mcp-servers/mcp-server-filesystem/dist/index.js /tmp/workspace",
        agent_id: Some("mcp"),
        role:     Some(ProcessRole::Server),
    });
}

#[test]
fn test_mcp_sequential_thinking() {
    assert_fixture(&Fixture {
        label:    "MCP sequential-thinking",
        cmd:      "node /Users/user/.npm/_npx/sequential-thinking/dist/index.js",
        agent_id: Some("mcp"),
        role:     Some(ProcessRole::Server),
    });
}

#[test]
fn test_mcp_playwright() {
    assert_fixture(&Fixture {
        label:    "Playwright MCP server",
        cmd:      "node /Users/user/.claude/mcp-servers/playwright-mcp/index.js --headless",
        agent_id: Some("mcp"),
        role:     Some(ProcessRole::Server),
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// Negative tests — things that must NOT match
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_negative_grep_claude() {
    assert_fixture(&Fixture {
        label:    "grep for claude (excluded)",
        cmd:      "grep -r claude src/",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_agentwatch_self() {
    assert_fixture(&Fixture {
        label:    "AgentWatch itself",
        cmd:      "/Applications/AgentWatch.app/Contents/MacOS/AgentWatch",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_ps_ax() {
    assert_fixture(&Fixture {
        label:    "ps ax pipe",
        cmd:      "ps ax | grep codex",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_random_node() {
    assert_fixture(&Fixture {
        label:    "random node server",
        cmd:      "node /Users/user/projects/my-api/dist/server.js --port 3000",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_vim() {
    assert_fixture(&Fixture {
        label:    "vim editor",
        cmd:      "/usr/bin/vim src/main.rs",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_bash() {
    assert_fixture(&Fixture {
        label:    "plain bash",
        cmd:      "/bin/bash --login",
        agent_id: None,
        role:     None,
    });
}

#[test]
fn test_negative_cursor_ui_service() {
    assert_fixture(&Fixture {
        label:    "CursorUIViewService (excluded)",
        cmd:      "/System/Library/Frameworks/UIKit.framework/CursorUIViewService",
        agent_id: None,
        role:     None,
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// Precedence tests — order-dependent matching
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_precedence_subagent_before_main() {
    let cmd = "/usr/local/bin/claude --input-format stream-json --output-format stream-json";
    let result = classify_process(cmd).expect("should match");
    assert_eq!(result.agent_id, "claude-subagent",
        "subagent fingerprint must take precedence over claude-code");
    assert_eq!(result.role, ProcessRole::Subagent);
}

#[test]
fn test_precedence_codex_app_before_cli() {
    let cmd = "node /path/to/codex-app-server --port 3000";
    let result = classify_process(cmd).expect("should match");
    assert_eq!(result.agent_id, "codex-app",
        "codex-app fingerprint must take precedence over codex-cli");
}

#[test]
fn test_precedence_claude_code_not_desktop() {
    let cmd = "/usr/local/bin/claude -- help me debug this";
    let result = classify_process(cmd).expect("should match");
    assert_eq!(result.agent_id, "claude-code",
        "claude-code should match, not claude-desktop");
}

// ═══════════════════════════════════════════════════════════════════════════
// Exclusion-rule tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_exclude_claude_app_from_claude_code() {
    let cmd = "/Applications/Claude.app/Contents/MacOS/Claude --type=renderer";
    let result = classify_process(cmd).expect("should match something");
    assert_ne!(result.agent_id, "claude-code",
        "Claude.app should be excluded from claude-code fingerprint");
    assert_eq!(result.agent_id, "claude-desktop");
}

#[test]
fn test_exclude_codex_app_from_codex_cli() {
    let cmd = "node /path/to/codex app-server --model o3";
    let result = classify_process(cmd).expect("should match");
    assert_ne!(result.agent_id, "codex-cli",
        "'codex app' should be excluded from codex-cli fingerprint");
}

// ═══════════════════════════════════════════════════════════════════════════
// matches_patterns unit tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_matches_patterns_single_required() {
    assert!(matches_patterns("hello world", &["hello"]));
    assert!(!matches_patterns("hello world", &["xyz"]));
}

#[test]
fn test_matches_patterns_multiple_required_and() {
    assert!(matches_patterns("abc def ghi", &["abc", "ghi"]));
    assert!(!matches_patterns("abc def ghi", &["abc", "xyz"]));
}

#[test]
fn test_matches_patterns_pipe_or() {
    assert!(matches_patterns("hello world", &["hello|goodbye"]));
    assert!(matches_patterns("goodbye world", &["hello|goodbye"]));
    assert!(!matches_patterns("neutral world", &["hello|goodbye"]));
}

#[test]
fn test_matches_patterns_combined_and_or() {
    let patterns = &["cat|dog", "food|water"];
    assert!(matches_patterns("cat likes food", patterns));
    assert!(matches_patterns("dog drinks water", patterns));
    assert!(!matches_patterns("cat sleeps", patterns));
}

// ═══════════════════════════════════════════════════════════════════════════
// DetectionResult field tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_detection_result_fields() {
    let result = classify_process("/usr/local/bin/claude --verbose").unwrap();
    assert_eq!(result.agent_id, "claude-code");
    assert_eq!(result.vendor, "Anthropic");
    assert_eq!(result.icon, "◆");
    assert_eq!(result.color, "#cc785c");
    assert!(result.confidence >= 90);
    assert!(!result.match_reason.is_empty());
}

#[test]
fn test_detection_result_match_reason() {
    let result = classify_process("node /opt/@google/gemini-cli/bin/gemini --model pro").unwrap();
    assert_eq!(result.agent_id, "gemini");
    assert!(result.match_reason.iter().any(|r| r.contains("@google/gemini-cli")),
        "match_reason should contain the matched substring: {:?}", result.match_reason);
}

#[test]
fn test_detection_result_host_app() {
    let result = classify_process(
        "/Users/user/.claude/local/node_modules/.bin/claude --input-format stream-json"
    ).unwrap();
    assert_eq!(result.agent_id, "claude-subagent");
    assert_eq!(result.host_app, Some("Claude Code".to_string()));
}

#[test]
fn test_detection_result_no_host_app() {
    let result = classify_process("/usr/local/bin/claude -- do stuff").unwrap();
    assert_eq!(result.agent_id, "claude-code");
    assert_eq!(result.host_app, None);
}

// ═══════════════════════════════════════════════════════════════════════════
// Confidence tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_high_confidence_agents() {
    let claude = classify_process("/usr/local/bin/claude --verbose").unwrap();
    assert!(claude.confidence >= 90, "claude-code confidence should be >=90, got {}", claude.confidence);

    let codex = classify_process("node /path/@openai/codex/cli.js").unwrap();
    assert!(codex.confidence >= 90, "codex-cli confidence should be >=90, got {}", codex.confidence);

    let gemini = classify_process("node /path/@google/gemini-cli/index.js").unwrap();
    assert!(gemini.confidence >= 90, "gemini confidence should be >=90, got {}", gemini.confidence);
}

#[test]
fn test_lower_confidence_broad_matches() {
    let mcp = classify_process("node /path/to/mcp-server-whatever/dist/index.js").unwrap();
    assert!(mcp.confidence <= 75, "mcp confidence should be <=75, got {}", mcp.confidence);

    let vscode = classify_process("/path/to/Code Helper --type=extensionHost").unwrap();
    assert!(vscode.confidence <= 80, "vscode confidence should be <=80, got {}", vscode.confidence);
}

// ═══════════════════════════════════════════════════════════════════════════
// get_agent_db() integrity tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_agents_have_unique_ids() {
    let mut seen = std::collections::HashSet::new();
    for fp in get_agent_db() {
        assert!(seen.insert(fp.id),
            "duplicate agent id in get_agent_db(): '{}'", fp.id);
    }
}

#[test]
fn test_all_agents_have_names() {
    for fp in get_agent_db() {
        assert!(!fp.name.is_empty(), "agent '{}' has empty name", fp.id);
        assert!(!fp.icon.is_empty(), "agent '{}' has empty icon", fp.id);
        assert!(!fp.vendor.is_empty(), "agent '{}' has empty vendor", fp.id);
        assert!(!fp.color.is_empty(), "agent '{}' has empty color", fp.id);
    }
}

#[test]
fn test_all_agents_have_requires() {
    for fp in get_agent_db() {
        assert!(!fp.requires.is_empty(),
            "agent '{}' has no requires patterns — it would match everything", fp.id);
    }
}

#[test]
fn test_confidence_range() {
    for fp in get_agent_db() {
        assert!(fp.confidence > 0 && fp.confidence <= 100,
            "agent '{}' has invalid confidence: {} (must be 1..=100)", fp.id, fp.confidence);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Session grouping tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_session_key_with_cwd() {
    let key = generate_session_key("claude-code", "/Users/user/project", Some(1234), None);
    assert_eq!(key, "claude-code:/Users/user/project");
}

#[test]
fn test_session_key_without_cwd() {
    let key = generate_session_key("claude-code", "", Some(1234), None);
    assert_eq!(key, "claude-code:pid:1234");
}

#[test]
fn test_session_key_groups_same_agent_same_dir() {
    let key1 = generate_session_key("claude-subagent", "/proj", Some(100), Some("Claude Code"));
    let key2 = generate_session_key("claude-subagent", "/proj", Some(200), Some("Claude Code"));
    assert_eq!(key1, key2, "same agent + same cwd should produce the same session key");
}

#[test]
fn test_session_key_separates_different_dirs() {
    let key1 = generate_session_key("claude-code", "/proj-a", None, None);
    let key2 = generate_session_key("claude-code", "/proj-b", None, None);
    assert_ne!(key1, key2, "different cwds should produce different session keys");
}

#[test]
fn test_session_key_subagent_resolves_to_parent() {
    let parent_key = generate_session_key("claude-code", "/proj", Some(100), None);
    let child_key  = generate_session_key("claude-subagent", "/proj", Some(200), Some("Claude Code"));
    assert_eq!(parent_key, child_key,
        "subagent should resolve to parent's session key via host_app");
}

#[test]
fn test_session_key_codex_sandbox_resolves_to_parent() {
    let parent_key = generate_session_key("codex-app", "/proj", Some(100), None);
    let child_key  = generate_session_key("codex-sandbox", "/proj", Some(200), Some("Codex App"));
    assert_eq!(parent_key, child_key,
        "codex-sandbox should resolve to codex-app's session key");
}

#[test]
fn test_session_key_cursor_helper_resolves() {
    let parent_key = generate_session_key("cursor-agent", "/proj", None, None);
    let child_key  = generate_session_key("continue", "/proj", Some(300), Some("Cursor"));
    assert_eq!(parent_key, child_key,
        "Continue helper in Cursor should share Cursor's session key");
}

#[test]
fn test_session_key_vscode_helper_resolves() {
    let parent_key = generate_session_key("vscode-agent", "/proj", None, None);
    let child_key  = generate_session_key("continue", "/proj", Some(400), Some("VS Code"));
    assert_eq!(parent_key, child_key,
        "Continue helper in VS Code should share VS Code's session key");
}

#[test]
fn test_session_key_vscode_cursor_dual_resolves() {
    let parent_key = generate_session_key("vscode-agent", "/proj", None, None);
    let child_key  = generate_session_key("continue", "/proj", Some(500), Some("VS Code / Cursor"));
    assert_eq!(parent_key, child_key,
        "VS Code / Cursor host should resolve to vscode-agent");
}

#[test]
fn test_session_key_windsurf_helper_resolves() {
    let parent_key = generate_session_key("windsurf", "/proj", None, None);
    let child_key  = generate_session_key("codeium-worker", "/proj", Some(600), Some("Windsurf"));
    assert_eq!(parent_key, child_key,
        "Windsurf child should share Windsurf's session key");
}

#[test]
fn test_session_key_no_host_app_stays_independent() {
    let key = generate_session_key("mcp", "/proj", Some(700), None);
    assert_eq!(key, "mcp:/proj", "no host_app = no resolution, keeps own agent_id");
}

#[test]
fn test_session_key_unknown_host_app_stays_independent() {
    let key = generate_session_key("some-agent", "/proj", Some(800), Some("Unknown App"));
    assert_eq!(key, "some-agent:/proj", "unknown host_app = no resolution");
}

// ═══════════════════════════════════════════════════════════════════════════
// Gemini Homebrew path test
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_gemini_homebrew() {
    assert_fixture(&Fixture {
        label:    "Gemini via Homebrew (Apple Silicon)",
        cmd:      "node --no-warnings=DEP0040 /opt/homebrew/bin/gemini",
        agent_id: Some("gemini"),
        role:     Some(ProcessRole::Main),
    });
}

#[test]
fn test_gemini_homebrew_intel() {
    assert_fixture(&Fixture {
        label:    "Gemini via Homebrew (Intel Mac)",
        cmd:      "node --no-warnings=DEP0040 /usr/local/bin/gemini --model gemini-2.5-pro",
        agent_id: Some("gemini"),
        role:     Some(ProcessRole::Main),
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// adopt_children field tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_adopt_children_enabled_for_cli_agents() {
    let adopters = ["claude-code", "codex-cli", "gemini", "commander", "openclaw"];
    for agent_id in &adopters {
        let fp = get_agent_db().iter().find(|f| f.id == *agent_id)
            .unwrap_or_else(|| panic!("agent '{}' not found in DB", agent_id));
        assert!(fp.adopt_children,
            "agent '{}' should have adopt_children=true", agent_id);
    }
}

#[test]
fn test_adopt_children_disabled_for_ide_hosts() {
    let non_adopters = ["cursor-agent", "vscode-agent", "windsurf", "claude-desktop",
                        "codex-app", "codex-sandbox", "claude-subagent", "mcp"];
    for agent_id in &non_adopters {
        let fp = get_agent_db().iter().find(|f| f.id == *agent_id)
            .unwrap_or_else(|| panic!("agent '{}' not found in DB", agent_id));
        assert!(!fp.adopt_children,
            "agent '{}' should have adopt_children=false", agent_id);
    }
}

#[test]
fn test_adopt_children_detection_result_propagation() {
    let codex = classify_process("node /path/@openai/codex/cli.js --task build").unwrap();
    assert!(codex.adopt_children, "codex-cli DetectionResult should have adopt_children=true");

    let desktop = classify_process("/Applications/Claude.app/Contents/MacOS/Claude Helper --type=renderer").unwrap();
    assert!(!desktop.adopt_children, "claude-desktop DetectionResult should have adopt_children=false");
}
