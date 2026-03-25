# 🔍 AgentWatch

**Monitor AI coding agents from the terminal.**

Detects and monitors AI coding agents like Claude, Cursor, Copilot, Codeium, and others running on your system.

[![Crates.io](https://img.shields.io/crates/v/agentwatch.svg)](https://crates.io/crates/agentwatch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Installation

```bash
cargo install agentwatch
```

## Usage

```bash
# List all detected AI agents
agentwatch

# Show summary statistics
agentwatch stats

# Real-time monitoring (htop-style)
agentwatch watch

# Output as JSON (for scripting)
agentwatch -f json

# Kill zombie agents (idle > 30 minutes)
agentwatch kill --dry-run
```

## Example Output

```
🔍 AgentWatch (backend: core)

╭───────┬──────────────────────────────┬──────────┬───────┬───────┬──────────┬────────┬─────────────╮
│ PID   │ AGENT                        │ ROLE     │ CPU%  │ MEM   │ UPTIME   │ STATUS │ PROJECT     │
├───────┼──────────────────────────────┼──────────┼───────┼───────┼──────────┼────────┼─────────────┤
│ 34601 │ ◆ Claude Code (Anthropic)    │ main     │ 33.4% │ 113MB │ 00:35:37 │ ●      │ my-project  │
│ 68983 │ ⌘ Cursor (Cursor)            │ host_app │ 23.1% │ 188MB │ 00:17:25 │ ●      │ webapp      │
│ 14984 │ ⬡ Copilot (GitHub)           │ helper   │ 0.4%  │ 181MB │ 16:04:13 │ ○      │ api-server  │
╰───────┴──────────────────────────────┴──────────┴───────┴───────┴──────────┴────────┴─────────────╯
```

## Supported Agents

| Agent | Vendor | Icon |
|-------|--------|------|
| Claude (Desktop, Code, Subagent) | Anthropic | ◆ |
| Cursor | Cursor | ⌘ |
| Copilot | GitHub | ⬡ |
| Codeium / Windsurf | Codeium | 〜 |
| ChatGPT / Codex | OpenAI | ◈ |
| Gemini | Google | ✦ |
| Aider | Aider | ⚡ |
| Cody | Sourcegraph | ❖ |
| Continue | Continue | ▸ |
| Amazon Q | AWS | ◊ |
| OpenClaw | OpenClaw | ✶ |

## Packages

| Crate | Description |
|-------|-------------|
| [`agentwatch`](https://crates.io/crates/agentwatch) | CLI tool |
| [`agentwatch-core`](https://crates.io/crates/agentwatch-core) | Detection library |

## Use as a Library

```toml
[dependencies]
agentwatch-core = "0.1"
```

```rust
use agentwatch_core::{scan_agents, calculate_stats};

fn main() -> agentwatch_core::Result<()> {
    let agents = scan_agents()?;
    println!("Found {} AI agents", agents.len());
    
    for agent in agents {
        println!("{} {} ({})", agent.icon, agent.name, agent.vendor);
    }
    
    Ok(())
}
```

## Desktop & Mobile Apps

For a full GUI experience with history and token cost tracking:

🖥️ **[AgentWatch Desktop](https://agentwatch.tools)** — macOS app  
📱 **AgentWatch Mobile** — Coming soon

## License

MIT © [Calcura Software LLC](https://agentwatch.tools)
