#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod echo;

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod touch;

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod sudo;

#[cfg(any(target_os = "macos", target_os = "windows"))]
pub mod disown;

// Windows-only modules
#[cfg(target_os = "windows")]
pub mod chmod;
#[cfg(target_os = "windows")]
pub mod chown;
#[cfg(target_os = "windows")]
pub mod pipeline;
#[cfg(target_os = "windows")]
pub mod powershell;
#[cfg(target_os = "windows")]
pub mod cd;
#[cfg(target_os = "windows")]
pub mod df;
#[cfg(target_os = "windows")]
pub mod free;
#[cfg(target_os = "windows")]
pub mod git;
#[cfg(target_os = "windows")]
pub mod process;
#[cfg(target_os = "windows")]
pub mod ps;
#[cfg(target_os = "windows")]
pub mod sensors;
#[cfg(target_os = "windows")]
pub mod tui;
#[cfg(target_os = "windows")]
pub mod uname;
#[cfg(target_os = "windows")]
pub mod uptime;
#[cfg(windows)]
pub mod kill;
#[cfg(target_os = "windows")]
pub mod job;

// src/command/mod.rs
pub fn dummy() {
  println!("command module loaded");
}

pub async fn grep_async_from_string(pattern: &str, content: &str) -> io::Result<String> {
  let result = content
      .lines()
      .filter(|line| line.contains(pattern))
      .map(|s| format!("{}\n", s))
      .collect();
  Ok(result)
}

pub async fn head_async_from_string(content: &str, lines: usize) -> io::Result<String> {
  let result = content
      .lines()
      .take(lines)
      .map(|s| format!("{}\n", s))
      .collect();
  Ok(result)
}
