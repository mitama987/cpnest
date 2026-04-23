//! Smoke probe: spawn copilot via our PtyHandle (so DSR replies are
//! wired up), read its output for 5s, kill, and print a summary. Used to
//! verify ConPTY + copilot interaction end-to-end without the full TUI.
//!
//! Run with: `cargo run --example spawn_probe`

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use cpnest::copilot::launcher::spawn_copilot;

fn main() {
    let parser = Arc::new(Mutex::new(vt100::Parser::new(40, 140, 2000)));
    let cwd = std::env::current_dir().expect("cwd");

    let (pty, cmd_label, copilot_running) =
        spawn_copilot(&cwd, Arc::clone(&parser)).expect("spawn_copilot");
    println!("spawned: {cmd_label} (copilot_running={copilot_running})");

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(200));
    }

    pty.kill();
    thread::sleep(Duration::from_millis(300));

    let g = parser.lock().unwrap();
    let screen = g.screen();
    let contents = screen.contents();
    let visible_len = contents.chars().filter(|c| !c.is_whitespace()).count();
    println!("visible (non-ws) chars on screen: {visible_len}");
    println!("--- screen dump (first 40 lines) ---");
    for (i, line) in contents.lines().take(40).enumerate() {
        println!("{:>2}: {}", i + 1, line);
    }
}
