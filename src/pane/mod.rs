pub mod grid;
pub mod pty;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::Result;

use crate::copilot::launcher::spawn_copilot;

pub type PaneId = u64;

pub struct Pane {
    pub id: PaneId,
    pub cwd: PathBuf,
    pub created_at: Instant,
    pub pty: pty::PtyHandle,
    pub parser: Arc<Mutex<vt100::Parser>>,
    pub command: String,
    pub copilot_running: bool,
}

impl Pane {
    pub fn spawn(id: PaneId, cwd: &Path) -> Result<Self> {
        let parser = Arc::new(Mutex::new(vt100::Parser::new(24, 80, 2000)));
        let (pty, command, copilot_running) = spawn_copilot(cwd, Arc::clone(&parser))?;
        Ok(Self {
            id,
            cwd: cwd.to_path_buf(),
            created_at: Instant::now(),
            pty,
            parser,
            command,
            copilot_running,
        })
    }

    pub fn resize(&self, rows: u16, cols: u16) {
        self.pty.resize(rows, cols);
        if let Ok(mut p) = self.parser.lock() {
            p.set_size(rows, cols);
        }
    }

    pub fn write(&self, data: &[u8]) {
        self.pty.write(data);
    }

    pub fn terminate(self) {
        self.pty.kill();
    }
}
