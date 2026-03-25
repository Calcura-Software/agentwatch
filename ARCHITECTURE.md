# AgentWatch Architecture

> Unified architecture for the AgentWatch ecosystem: desktop app, CLI, and shared detection library.

## Overview

AgentWatch uses a modular architecture with a shared core library that powers both the desktop application and the command-line interface.

```
┌─────────────────────────────────────────────────────────────────┐
│                      AgentWatch Ecosystem                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────────────┐          ┌──────────────────┐            │
│   │  agentwatch-cli  │          │    agentwatch    │            │
│   │   (Terminal)     │          │   (Tauri App)    │            │
│   │                  │          │                  │            │
│   │  cargo install   │          │  Desktop GUI     │            │
│   │  agentwatch      │          │  System tray     │            │
│   │                  │          │  Notifications   │            │
│   └────────┬─────────┘          └────────┬─────────┘            │
│            │                             │                       │
│            │         depends on          │                       │
│            └──────────────┬──────────────┘                       │
│                           ▼                                      │
│            ┌──────────────────────────┐                          │
│            │    agentwatch-core       │                          │
│            │   (Shared Library)       │                          │
│            │                          │                          │
│            │  • Process detection     │                          │
│            │  • Agent fingerprints    │                          │
│            │  • Classification        │                          │
│            │  • Statistics            │                          │
│            └──────────────────────────┘                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
agentwatch.tools/
├── agentwatch-core/       # Shared detection library (crates.io)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs         # Public API: scan_agents(), calculate_stats()
│       ├── error.rs       # Error types with thiserror
│       └── detection/     # Detection engine
│           ├── mod.rs
│           ├── types.rs       # ProcessRole, DetectionResult, AgentFingerprint
│           ├── fingerprint.rs # Pattern matching, classify_process()
│           ├── grouping.rs    # Session key generation
│           └── vendors/       # Per-vendor fingerprints
│               ├── anthropic.rs
│               ├── openai.rs
│               ├── google.rs
│               ├── cursor.rs
│               ├── microsoft.rs
│               ├── codeium.rs
│               └── others.rs
│
├── agentwatch-cli/        # Terminal client (crates.io)
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs        # CLI implementation with clap
│
├── agentwatch/            # Desktop app (Tauri)
│   ├── package.json       # Frontend (React/Solid)
│   └── src-tauri/
│       ├── Cargo.toml     # Depends on agentwatch-core
│       └── src/
│           └── lib.rs     # Tauri commands, re-exports core
│
└── agentwatch-mobile/     # Mobile companion (future)
```

## Components

### agentwatch-core

The shared detection library. Published to crates.io for external use.

**Key exports:**
```rust
// Scan running processes for AI agents
pub fn scan_agents() -> Result<Vec<AgentProcess>>;

// Calculate summary statistics
pub fn calculate_stats(agents: &[AgentProcess]) -> AgentStats;

// Direct access to detection engine
pub use detection::{classify_process, get_agent_db, generate_session_key};
```

**Features:**
- Zero-copy fingerprint database with `OnceLock`
- Pattern matching with OR (`|`) and AND (multiple patterns)
- Role classification: Main, Subagent, Helper, Wrapper, HostApp, Server
- Confidence scoring (0-100)
- Session key generation for grouping related processes

**Dependencies:**
- `sysinfo` 0.38 — Cross-platform process information
- `serde` — Serialization for IPC
- `thiserror` — Ergonomic error types
- `regex` + `lazy_static` — Pattern compilation

### agentwatch-cli

Terminal interface for quick checks and scripting.

**Installation:**
```bash
cargo install agentwatch
```

**Commands:**
```bash
agentwatch              # List detected agents
agentwatch stats        # Summary statistics
agentwatch watch        # Real-time monitoring
agentwatch kill         # Clean up zombies
agentwatch compare      # Compare detection backends
```

**Output formats:**
- `--format table` (default) — Human-readable table
- `--format json` — Machine-readable JSON
- `--format minimal` — Tab-separated for scripting

**Backend selection:**
```bash
agentwatch --backend core        # Use agentwatch-core (default)
agentwatch --backend standalone  # Use built-in detection (simpler)
agentwatch compare               # Compare both side-by-side
```

### agentwatch (Tauri App)

Full-featured desktop application with GUI.

**Integration with core:**
```rust
// src-tauri/Cargo.toml
[dependencies]
agentwatch-core = { path = "../agentwatch-core" }

// src-tauri/src/lib.rs
pub use agentwatch_core::detection;
```

**Features beyond CLI:**
- System tray with live agent count
- Session history and timeline
- Token cost estimation
- Desktop notifications
- Menu bar quick actions

---

## Detection Engine

### How Detection Works

1. **Scan:** `sysinfo` enumerates all running processes
2. **Extract:** Command line, working directory, PID, resource usage
3. **Classify:** Match against fingerprint database (first match wins)
4. **Enrich:** Add role, confidence, host app, match reasons
5. **Group:** Generate session keys for related processes

### Fingerprint Structure

```rust
pub struct AgentFingerprint {
    pub id:         &'static str,   // "claude-code"
    pub name:       &'static str,   // "Claude Code"
    pub vendor:     &'static str,   // "Anthropic"
    pub icon:       &'static str,   // "◆"
    pub color:      &'static str,   // "#D97706"
    pub role:       ProcessRole,    // Main, Subagent, etc.
    pub confidence: u8,             // 0-100
    pub requires:   &'static [&'static str],  // Patterns (AND)
    pub excludes:   &'static [&'static str],  // Anti-patterns
    pub host_app:   Option<&'static str>,     // Parent app
    pub adopt_children: bool,       // Group child processes
}
```

### Pattern Matching

- **OR within pattern:** `"cursor|Cursor"` matches either
- **AND across patterns:** `["node", "copilot"]` requires both
- **Excludes:** Skip false positives

Example:
```rust
AgentFingerprint {
    id: "claude-code",
    requires: &["/bin/claude|claude-code|/claude"],
    excludes: &["--input-format stream-json"],  // That's subagent
    // ...
}
```

### Process Roles

| Role | Description | Example |
|------|-------------|---------|
| `Main` | Primary user session | `claude --dangerously-skip-permissions` |
| `Subagent` | Worker spawned by main | `claude --input-format stream-json` |
| `Helper` | Utility process | Language servers, formatters |
| `Wrapper` | OS-level container | `sandbox-exec` |
| `HostApp` | IDE hosting agents | Cursor, VS Code |
| `Server` | Long-running daemon | MCP servers |

---

## Installation

### CLI (Recommended for scripts)

```bash
# From crates.io
cargo install agentwatch

# From source
git clone https://github.com/ArtificialDiamond/agentwatch.git
cd agentwatch/agentwatch-cli
cargo install --path .
```

### Desktop App

```bash
# Development
cd agentwatch
pnpm install
pnpm tauri dev

# Build release
pnpm tauri build
```

### As a Library

```toml
# Cargo.toml
[dependencies]
agentwatch-core = "0.1"
```

```rust
use agentwatch_core::{scan_agents, calculate_stats};

fn main() -> agentwatch_core::Result<()> {
    let agents = scan_agents()?;
    let stats = calculate_stats(&agents);
    
    println!("Found {} agents ({} active)", stats.total, stats.active);
    
    for agent in agents {
        println!("{}: {} ({})", agent.pid, agent.name, agent.vendor);
    }
    
    Ok(())
}
```

---

## Development

### Running Tests

```bash
# Core library (75 tests)
cd agentwatch-core
cargo test

# CLI
cd agentwatch-cli
cargo test

# All with coverage
cargo tarpaulin --all
```

### Code Quality

```bash
# Lint
cargo clippy --all-targets

# Format
cargo fmt --all

# Check all crates
for d in agentwatch-core agentwatch-cli agentwatch/src-tauri; do
  (cd $d && cargo check)
done
```

### Adding a New Agent

1. Find the right vendor file in `agentwatch-core/src/detection/vendors/`
2. Add a new `AgentFingerprint` entry
3. Add test cases in `detection/tests.rs`
4. Run `cargo test` to verify

Example:
```rust
// vendors/others.rs
AgentFingerprint {
    id: "my-agent",
    name: "My Agent",
    vendor: "My Company",
    icon: "★",
    color: "#FF5733",
    role: ProcessRole::Main,
    confidence: 90,
    requires: &["my-agent", "--run"],
    excludes: &[],
    host_app: None,
    adopt_children: false,
},
```

---

## Roadmap

- [x] Extract `agentwatch-core` as shared library
- [x] CLI with multiple output formats
- [x] Error handling with `thiserror`
- [x] Update `sysinfo` to 0.38
- [ ] Add `tracing` for logging
- [ ] Async support for `scan_agents()`
- [ ] GitHub Actions CI/CD
- [ ] Publish to crates.io

---

## License

MIT © [Calcura Software LLC](https://agentwatch.tools)
