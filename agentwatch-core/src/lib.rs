//! AgentWatch Core — AI Agent Detection Library
//!
//! This crate provides the core detection logic for identifying AI coding agents
//! running on a system. It's used by both the AgentWatch desktop app and CLI.
//!
//! # Example
//!
//! ```no_run
//! use agentwatch_core::{scan_agents, AgentProcess};
//!
//! let agents = scan_agents().expect("failed to scan");
//! for agent in agents {
//!     println!("{}: {} ({})", agent.pid, agent.name, agent.vendor);
//! }
//! ```

pub mod detection;
pub mod error;

use detection::{classify_process, ProcessRole};
use serde::{Deserialize, Serialize};
use sysinfo::System;

// Re-exports
pub use detection::{AgentFingerprint, get_agent_db, matches_patterns, generate_session_key};
pub use error::{Error, Result};

/// Represents a detected AI agent process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProcess {
    pub pid: u32,
    pub name: String,
    pub vendor: String,
    pub icon: String,
    pub color: String,
    pub cpu: f32,
    pub mem_mb: f32,
    pub uptime_secs: u64,
    pub status: String,
    pub role: ProcessRole,
    pub confidence: u8,
    pub host_app: Option<String>,
    pub cwd: String,
    pub project: String,
    pub command: String,
    pub match_reason: Vec<String>,
}

/// Extract project name from working directory path
pub fn extract_project(cwd: &str) -> String {
    cwd.split('/')
        .rfind(|s| !s.is_empty())
        .unwrap_or("-")
        .to_string()
}

/// Format uptime seconds into human-readable string
pub fn format_uptime(secs: u64) -> String {
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    
    if d > 0 {
        format!("{}d {:02}:{:02}:{:02}", d, h, m, s)
    } else if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

/// Scan the system for AI coding agents
///
/// Returns a vector of detected agent processes, sorted by CPU usage (descending).
///
/// # Errors
///
/// Currently infallible, but returns `Result` for API stability and future
/// extensibility (e.g., permission errors, sandboxing restrictions).
pub fn scan_agents() -> Result<Vec<AgentProcess>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Brief pause for accurate CPU readings
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_all();

    let mut agents = Vec::new();

    for (pid, process) in sys.processes() {
        let cmd: String = process.cmd()
            .iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        
        if let Some(detection) = classify_process(&cmd) {
            let cwd = process.cwd()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let cpu = process.cpu_usage();
            let status = if cpu > 1.0 { "ACTIVE" } else { "IDLE" }.to_string();

            agents.push(AgentProcess {
                pid: pid.as_u32(),
                name: detection.name,
                vendor: detection.vendor,
                icon: detection.icon,
                color: detection.color,
                cpu,
                mem_mb: process.memory() as f32 / 1024.0 / 1024.0,
                uptime_secs: process.run_time(),
                status,
                role: detection.role,
                confidence: detection.confidence,
                host_app: detection.host_app,
                project: extract_project(&cwd),
                cwd,
                command: cmd.chars().take(200).collect(),
                match_reason: detection.match_reason,
            });
        }
    }

    // Sort by CPU usage descending
    agents.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
    Ok(agents)
}

/// Summary statistics for detected agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStats {
    pub total: usize,
    pub active: usize,
    pub idle: usize,
    pub total_cpu: f32,
    pub total_mem_mb: f32,
    pub by_vendor: std::collections::HashMap<String, usize>,
}

/// Calculate summary statistics for a list of agents
pub fn calculate_stats(agents: &[AgentProcess]) -> AgentStats {
    let mut by_vendor = std::collections::HashMap::new();
    
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_project() {
        assert_eq!(extract_project("/Users/dev/projects/myapp"), "myapp");
        assert_eq!(extract_project("/home/user/code/"), "code");
        assert_eq!(extract_project(""), "-");
    }

    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(30), "00:30");
        assert_eq!(format_uptime(3661), "01:01:01");
        assert_eq!(format_uptime(90061), "1d 01:01:01");
    }

    #[test]
    fn test_classify_claude() {
        let result = classify_process("/usr/local/bin/claude --dangerously-skip-permissions");
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.vendor, "Anthropic");
    }
}
