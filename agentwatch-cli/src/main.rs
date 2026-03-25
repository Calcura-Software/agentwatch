//! AgentWatch CLI — Monitor AI coding agents from the terminal
//!
//! Supports two detection backends:
//! - `core`: Uses agentwatch-core shared library (default)
//! - `standalone`: Built-in detection (for comparison/testing)

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tabled::{Table, Tabled, settings::Style};

#[cfg(unix)]

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Detection Backend Abstraction
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
enum Backend {
    /// Use agentwatch-core shared library
    #[default]
    Core,
    /// Use standalone built-in detection
    Standalone,
}

/// Unified agent process representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProcess {
    pub pid: u32,
    pub name: String,
    pub vendor: String,
    pub icon: String,
    pub role: String,
    pub cpu: f32,
    pub mem_mb: f32,
    pub uptime_secs: u64,
    pub status: String,
    pub project: String,
    pub confidence: u8,
    pub backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStats {
    pub total: usize,
    pub active: usize,
    pub idle: usize,
    pub total_cpu: f32,
    pub total_mem_mb: f32,
    pub by_vendor: HashMap<String, usize>,
    pub backend: String,
}

// ── Core backend ─────────────────────────────────────────────────────────────

#[cfg(feature = "core")]
mod core_backend {
    use super::*;
    
    pub fn scan() -> Vec<AgentProcess> {
        match agentwatch_core::scan_agents() {
            Ok(agents) => agents
                .into_iter()
                .map(|a| AgentProcess {
                    pid: a.pid,
                    name: a.name,
                    vendor: a.vendor,
                    icon: a.icon,
                    role: format!("{}", a.role),
                    cpu: a.cpu,
                    mem_mb: a.mem_mb,
                    uptime_secs: a.uptime_secs,
                    status: a.status,
                    project: a.project,
                    confidence: a.confidence,
                    backend: "core".to_string(),
                })
                .collect(),
            Err(e) => {
                eprintln!("{}: {}", "Error scanning agents".red(), e);
                vec![]
            }
        }
    }
    
    pub fn format_uptime(secs: u64) -> String {
        agentwatch_core::format_uptime(secs)
    }
}

// ── Standalone backend ───────────────────────────────────────────────────────

#[cfg(feature = "standalone")]
mod standalone_backend {
    use super::*;
    use sysinfo::System;
    
    const AGENT_PATTERNS: &[(&str, &str, &str, &str)] = &[
        ("claude", "Claude", "Anthropic", "◆"),
        ("/bin/claude", "Claude Code", "Anthropic", "◆"),
        ("--input-format stream-json", "Claude Subagent", "Anthropic", "◇"),
        ("Claude.app", "Claude Desktop", "Anthropic", "◆"),
        ("cursor", "Cursor", "Cursor", "⌘"),
        ("copilot", "Copilot", "GitHub", "⬡"),
        ("codeium", "Codeium", "Codeium", "〜"),
        ("windsurf", "Windsurf", "Codeium", "〜"),
        ("codex", "Codex", "OpenAI", "◈"),
        ("openai", "OpenAI", "OpenAI", "◈"),
        ("gemini", "Gemini", "Google", "✦"),
        ("aider", "Aider", "Aider", "⚡"),
        ("openclaw", "OpenClaw", "OpenClaw", "✶"),
        ("mcp-server", "MCP Server", "Various", "⬡"),
    ];

    fn detect_agent(cmd: &str) -> Option<(&'static str, &'static str, &'static str)> {
        let cmd_lower = cmd.to_lowercase();
        for (pattern, name, vendor, icon) in AGENT_PATTERNS {
            if cmd_lower.contains(&pattern.to_lowercase()) {
                return Some((name, vendor, icon));
            }
        }
        None
    }

    pub fn scan() -> Vec<AgentProcess> {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();

        let mut agents = Vec::new();

        for (pid, process) in sys.processes() {
            let cmd: String = process.cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            
            if let Some((name, vendor, icon)) = detect_agent(&cmd) {
                let cwd = process.cwd()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                let cpu = process.cpu_usage();
                let status = if cpu > 1.0 { "ACTIVE" } else { "IDLE" }.to_string();
                let project = cwd.split('/').rfind(|s| !s.is_empty()).unwrap_or("-").to_string();

                agents.push(AgentProcess {
                    pid: pid.as_u32(),
                    name: name.to_string(),
                    vendor: vendor.to_string(),
                    icon: icon.to_string(),
                    role: "unknown".to_string(),
                    cpu,
                    mem_mb: process.memory() as f32 / 1024.0 / 1024.0,
                    uptime_secs: process.run_time(),
                    status,
                    project,
                    confidence: 80,
                    backend: "standalone".to_string(),
                });
            }
        }

        agents.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
        agents
    }
    
    pub fn format_uptime(secs: u64) -> String {
        let d = secs / 86400;
        let h = (secs % 86400) / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        if d > 0 { format!("{}d {:02}:{:02}:{:02}", d, h, m, s) }
        else if h > 0 { format!("{:02}:{:02}:{:02}", h, m, s) }
        else { format!("{:02}:{:02}", m, s) }
    }
}

// ── Unified interface ────────────────────────────────────────────────────────

fn scan_agents(backend: Backend) -> Vec<AgentProcess> {
    match backend {
        #[cfg(feature = "core")]
        Backend::Core => core_backend::scan(),
        #[cfg(feature = "standalone")]
        Backend::Standalone => standalone_backend::scan(),
        #[allow(unreachable_patterns)]
        _ => {
            eprintln!("{}", "Backend not available. Compile with --features core or --features standalone".red());
            vec![]
        }
    }
}

fn format_uptime(secs: u64, backend: Backend) -> String {
    match backend {
        #[cfg(feature = "core")]
        Backend::Core => core_backend::format_uptime(secs),
        #[cfg(feature = "standalone")]
        Backend::Standalone => standalone_backend::format_uptime(secs),
        #[allow(unreachable_patterns)]
        _ => format!("{}s", secs),
    }
}

fn calculate_stats(agents: &[AgentProcess], backend: Backend) -> AgentStats {
    let mut by_vendor = HashMap::new();
    for agent in agents {
        *by_vendor.entry(agent.vendor.clone()).or_insert(0) += 1;
    }
    
    AgentStats {
        total: agents.len(),
        active: agents.iter().filter(|a| a.status == "ACTIVE").count(),
        idle: agents.iter().filter(|a| a.status == "IDLE").count(),
        total_cpu: agents.iter().map(|a| a.cpu).sum(),
        total_mem_mb: agents.iter().map(|a| a.mem_mb).sum(),
        by_vendor,
        backend: format!("{:?}", backend).to_lowercase(),
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// CLI Interface
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Parser)]
#[command(name = "agentwatch")]
#[command(author = "Calcura Software LLC <hello@agentwatch.tools>")]
#[command(version)]
#[command(about = "Monitor AI coding agents from the terminal", long_about = None)]
#[command(after_help = "Examples:
  agentwatch                    List agents (using core backend)
  agentwatch --backend standalone   Use standalone detection
  agentwatch compare            Compare both backends side-by-side
  agentwatch -f json            Output as JSON

More info: https://agentwatch.tools")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Detection backend to use
    #[arg(short, long, default_value = "core", global = true)]
    backend: Backend,

    /// Output format: table, json, minimal
    #[arg(short, long, default_value = "table", global = true)]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// List all detected AI agents (default)
    List,
    /// Show summary statistics
    Stats,
    /// Compare both detection backends
    Compare,
    /// Watch agents in real-time
    Watch {
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },
    /// Kill zombie/idle agents
    Kill {
        #[arg(short, long, default_value = "30")]
        idle_mins: u64,
        #[arg(long)]
        dry_run: bool,
    },
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Output
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Tabled)]
struct AgentRow {
    #[tabled(rename = "PID")]
    pid: u32,
    #[tabled(rename = "AGENT")]
    agent: String,
    #[tabled(rename = "ROLE")]
    role: String,
    #[tabled(rename = "CPU%")]
    cpu: String,
    #[tabled(rename = "MEM")]
    mem: String,
    #[tabled(rename = "UPTIME")]
    uptime: String,
    #[tabled(rename = "STATUS")]
    status: String,
    #[tabled(rename = "PROJECT")]
    project: String,
}

fn print_table(agents: &[AgentProcess], backend: Backend) {
    if agents.is_empty() {
        println!("{}", "No AI agents detected.".yellow());
        return;
    }

    let rows: Vec<AgentRow> = agents.iter().map(|a| {
        let status_icon = if a.status == "ACTIVE" { "●".green() } else { "○".dimmed() };
        AgentRow {
            pid: a.pid,
            agent: format!("{} {} ({})", a.icon, a.name, a.vendor),
            role: a.role.clone(),
            cpu: format!("{:.1}%", a.cpu),
            mem: format!("{:.0}MB", a.mem_mb),
            uptime: format_uptime(a.uptime_secs, backend),
            status: status_icon.to_string(),
            project: a.project.clone(),
        }
    }).collect();

    println!("{}", Table::new(rows).with(Style::rounded()));
}

fn print_stats(stats: &AgentStats) {
    println!("\n{} (backend: {})", "═══ AgentWatch Stats ═══".cyan().bold(), stats.backend.yellow());
    println!("\n  📊  {} total ({} active, {} idle)", 
        stats.total.to_string().bold(),
        stats.active.to_string().green(),
        stats.idle.to_string().yellow()
    );
    println!("  🔥  {:.1}% CPU", stats.total_cpu);
    println!("  💾  {:.0} MB RAM", stats.total_mem_mb);
    
    if !stats.by_vendor.is_empty() {
        println!("\n  By vendor:");
        let mut vendors: Vec<_> = stats.by_vendor.iter().collect();
        vendors.sort_by(|a, b| b.1.cmp(a.1));
        for (vendor, count) in vendors {
            println!("    → {}: {}", vendor, count);
        }
    }
    println!();
}

fn compare_backends() {
    println!("\n{}\n", "═══ Backend Comparison ═══".cyan().bold());
    
    #[cfg(feature = "core")]
    let core_agents = core_backend::scan();
    #[cfg(not(feature = "core"))]
    let core_agents: Vec<AgentProcess> = vec![];
    
    #[cfg(feature = "standalone")]
    let standalone_agents = standalone_backend::scan();
    #[cfg(not(feature = "standalone"))]
    let standalone_agents: Vec<AgentProcess> = vec![];

    println!("{}:", "CORE backend".green().bold());
    println!("  Total: {} agents", core_agents.len());
    let core_pids: std::collections::HashSet<_> = core_agents.iter().map(|a| a.pid).collect();
    
    println!("\n{}:", "STANDALONE backend".yellow().bold());
    println!("  Total: {} agents", standalone_agents.len());
    let standalone_pids: std::collections::HashSet<_> = standalone_agents.iter().map(|a| a.pid).collect();
    
    let only_core: Vec<_> = core_pids.difference(&standalone_pids).collect();
    let only_standalone: Vec<_> = standalone_pids.difference(&core_pids).collect();
    let both: Vec<_> = core_pids.intersection(&standalone_pids).collect();
    
    println!("\n{}:", "Differences".magenta().bold());
    println!("  ✓ Detected by both: {}", both.len());
    println!("  + Only in CORE: {} {:?}", only_core.len(), only_core.iter().take(5).collect::<Vec<_>>());
    println!("  - Only in STANDALONE: {} {:?}", only_standalone.len(), only_standalone.iter().take(5).collect::<Vec<_>>());
    
    // Show sample differences
    if !only_core.is_empty() {
        println!("\n  Sample CORE-only detections:");
        for pid in only_core.iter().take(3) {
            if let Some(a) = core_agents.iter().find(|a| a.pid == **pid) {
                println!("    PID {} → {} {} ({})", a.pid, a.icon, a.name, a.role);
            }
        }
    }
    println!();
}

fn watch_mode(interval: u64, backend: Backend) {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let now = chrono::Local::now().format("%H:%M:%S");
        println!("{} {} ({}s, backend: {})\n", 
            "🔍 AgentWatch".cyan().bold(), now.to_string().dimmed(), interval, format!("{:?}", backend).yellow()
        );
        let agents = scan_agents(backend);
        let stats = calculate_stats(&agents, backend);
        print_stats(&stats);
        print_table(&agents, backend);
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}

fn kill_zombies(idle_mins: u64, dry_run: bool, backend: Backend) {
    let agents = scan_agents(backend);
    let idle_secs = idle_mins * 60;
    let zombies: Vec<_> = agents.iter().filter(|a| a.status == "IDLE" && a.uptime_secs > idle_secs).collect();
    
    if zombies.is_empty() {
        println!("{}", "✓ No zombie agents found.".green());
        return;
    }
    
    println!("Found {} zombies (idle > {}min):\n", zombies.len().to_string().yellow(), idle_mins);
    for z in &zombies {
        let action = if dry_run { "Would kill" } else { "Killing" };
        println!("  {} PID {} — {} {}", action.yellow(), z.pid.to_string().bold(), z.icon, z.name);
        if !dry_run {
            #[cfg(unix)]
            unsafe { libc::kill(z.pid as i32, libc::SIGTERM); }
        }
    }
    if dry_run { println!("\n{}", "Run without --dry-run to kill.".dimmed()); }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Main
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn main() {
    let cli = Cli::parse();
    let backend = cli.backend;

    match cli.command {
        Some(Commands::Stats) => {
            let agents = scan_agents(backend);
            let stats = calculate_stats(&agents, backend);
            match cli.format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&stats).unwrap()),
                _ => print_stats(&stats),
            }
        }
        Some(Commands::Compare) => compare_backends(),
        Some(Commands::Watch { interval }) => watch_mode(interval, backend),
        Some(Commands::Kill { idle_mins, dry_run }) => kill_zombies(idle_mins, dry_run, backend),
        Some(Commands::List) | None => {
            let agents = scan_agents(backend);
            println!("\n{} (backend: {})\n", "🔍 AgentWatch".cyan().bold(), format!("{:?}", backend).yellow());
            match cli.format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&agents).unwrap()),
                "minimal" => for a in &agents { println!("{}\t{}\t{}\t{:.1}%", a.pid, a.name, a.role, a.cpu); },
                _ => print_table(&agents, backend),
            }
            println!();
        }
    }
}
