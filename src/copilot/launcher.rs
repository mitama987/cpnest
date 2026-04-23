use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use portable_pty::CommandBuilder;

use crate::pane::pty::PtyHandle;

/// Spawn a `copilot` process in a PTY, with a fallback to the system shell
/// when `copilot` is not available. Returns (handle, command-used, copilot-running?).
pub fn spawn_copilot(
    cwd: &Path,
    parser: Arc<Mutex<vt100::Parser>>,
) -> Result<(PtyHandle, String, bool)> {
    if let Some(bin) = resolve_copilot_bin() {
        let mut cmd = CommandBuilder::new(&bin);
        cmd.cwd(cwd);
        apply_env(&mut cmd);
        if let Ok(h) = PtyHandle::spawn(cmd, Arc::clone(&parser)) {
            let label = bin
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "copilot".to_string());
            return Ok((h, label, true));
        }
    }

    // Fallback: start the system shell so the pane is at least usable.
    let (h, shell) = spawn_shell(cwd, parser)
        .map_err(|e| anyhow!("failed to spawn both copilot and shell: {e}"))?;
    Ok((h, shell, false))
}

/// Spawn the system shell in a PTY (cmd.exe on Windows, `$SHELL` otherwise).
/// Used both as the initial fallback when `copilot` is missing and when Ctrl+C
/// 2 連打でペインを shell に切り戻すときの再起動先として使う。
pub fn spawn_shell(cwd: &Path, parser: Arc<Mutex<vt100::Parser>>) -> Result<(PtyHandle, String)> {
    let shell = if cfg!(windows) {
        std::env::var("ComSpec").unwrap_or_else(|_| "cmd.exe".to_string())
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
    };
    let mut cmd = CommandBuilder::new(&shell);
    cmd.cwd(cwd);
    apply_env(&mut cmd);
    let h = PtyHandle::spawn(cmd, parser).map_err(|e| anyhow!("failed to spawn shell: {e}"))?;
    Ok((h, shell))
}

fn apply_env(cmd: &mut CommandBuilder) {
    for (k, v) in std::env::vars() {
        cmd.env(k, v);
    }
    // Keep interactive TUIs happy on ConPTY.
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");
    cmd.env("FORCE_COLOR", "1");
    cmd.env("CI", "");
}

/// Locate the `copilot` executable by walking `PATH` with all plausible
/// Windows extensions. Honors `CPNEST_COPILOT_BIN` for manual override.
fn resolve_copilot_bin() -> Option<PathBuf> {
    if let Ok(custom) = std::env::var("CPNEST_COPILOT_BIN") {
        let p = PathBuf::from(custom);
        if p.is_file() {
            return Some(p);
        }
    }

    let exts: &[&str] = if cfg!(windows) {
        &["exe", "cmd", "bat", "ps1", ""]
    } else {
        &[""]
    };

    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        for ext in exts {
            let candidate = if ext.is_empty() {
                dir.join("copilot")
            } else {
                dir.join(format!("copilot.{ext}"))
            };
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}
