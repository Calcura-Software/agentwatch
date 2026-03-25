# 🔍 AgentWatch CLI

**Monitor AI coding agents from the terminal.**

Detects and monitors AI coding agents like Claude, Cursor, Copilot, Codeium, and others running on your system.

[![Crates.io](https://img.shields.io/crates/v/agentwatch.svg)](https://crates.io/crates/agentwatch)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- 🔍 **Detect** — Automatically finds AI agents running on your machine
- 📊 **Monitor** — CPU, memory, uptime, and project context
- ⚡ **Real-time** — Watch mode with configurable refresh interval
- 🧹 **Cleanup** — Kill zombie/idle agents automatically
- 📋 **Scriptable** — JSON output for CI/CD and automation

## Supported Agents

| Agent | Vendor |
|-------|--------|
| Claude (Desktop, Code) | Anthropic |
| Cursor | Cursor |
| Copilot | GitHub |
| Codeium / Windsurf | Codeium |
| ChatGPT / Codex | OpenAI |
| Gemini | Google |
| Aider | Aider |
| Cody | Sourcegraph |
| Continue | Continue |
| Amazon Q | AWS |
| Tabnine | Tabnine |
| OpenClaw | OpenClaw |

## Installation

### From crates.io

```bash
cargo install agentwatch
```

### From source

```bash
git clone https://github.com/ArtificialDiamond/agentwatch.git
cd agentwatch/agentwatch-cli
cargo install --path .
```

## Usage

```bash
# List all detected AI agents
agentwatch

# Show summary statistics
agentwatch stats

# Real-time monitoring (htop-style)
agentwatch watch

# Watch with custom interval (5 seconds)
agentwatch watch -i 5

# Output as JSON (for scripting)
agentwatch -f json

# Kill zombie agents (idle > 30 minutes)
agentwatch kill

# Preview which agents would be killed
agentwatch kill --dry-run

# Kill agents idle for more than 1 hour
agentwatch kill --idle-mins 60
```

## Example Output

```
🔍 AgentWatch

╭───────┬─────────────────────┬───────┬───────┬──────────┬────────┬─────────────╮
│ PID   │ AGENT               │ CPU%  │ MEM   │ UPTIME   │ STATUS │ PROJECT     │
├───────┼─────────────────────┼───────┼───────┼──────────┼────────┼─────────────┤
│ 34601 │ Claude (Anthropic)  │ 33.4% │ 113MB │ 35:37    │ ●      │ my-project  │
│ 68983 │ Cursor (Cursor)     │ 23.1% │ 188MB │ 17:25    │ ●      │ webapp      │
│ 14984 │ Copilot (GitHub)    │ 0.4%  │ 181MB │ 16:04:13 │ ○      │ api-server  │
╰───────┴─────────────────────┴───────┴───────┴──────────┴────────┴─────────────╯
```

## JSON Output

Perfect for scripting and CI/CD pipelines:

```bash
agentwatch -f json | jq '.[] | select(.status == "ACTIVE")'
```

```json
[
  {
    "pid": 34601,
    "name": "Claude",
    "vendor": "Anthropic",
    "cpu": 33.4,
    "mem_mb": 113.0,
    "uptime_secs": 2137,
    "status": "ACTIVE",
    "project": "my-project",
    "cwd": "/Users/dev/projects/my-project"
  }
]
```

## Desktop & Mobile Apps

For a full GUI experience with history, token cost tracking, and mobile companion app, check out:

🖥️ **[AgentWatch Desktop](https://agentwatch.tools)** — Free download for macOS  
📱 **AgentWatch Mobile** — Coming soon for iOS & Android

## License

MIT © [Calcura Software LLC](https://agentwatch.tools)
