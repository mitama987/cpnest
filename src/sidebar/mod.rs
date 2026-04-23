pub mod filetree;
pub mod git;
pub mod panelist;

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    FileTree = 0,
    Git = 1,
    Panes = 2,
}

impl Section {
    pub fn all() -> [Section; 3] {
        [Section::FileTree, Section::Git, Section::Panes]
    }
    pub fn title(self) -> &'static str {
        match self {
            Section::FileTree => "Files",
            Section::Git => "Git",
            Section::Panes => "Panes",
        }
    }
}

pub struct SidebarState {
    pub visible: bool,
    pub active: Section,
    pub cursors: [usize; 3],
    pub cwd: PathBuf,
    pub file_entries: Vec<filetree::Entry>,
    pub git_info: Option<git::GitInfo>,
}

impl SidebarState {
    pub fn new(cwd: PathBuf) -> Self {
        let file_entries = filetree::walk(&cwd, 3).unwrap_or_default();
        let git_info = git::load(&cwd).ok();
        Self {
            visible: true,
            active: Section::FileTree,
            cursors: [0; 3],
            cwd,
            file_entries,
            git_info,
        }
    }

    pub fn refresh(&mut self) {
        self.file_entries = filetree::walk(&self.cwd, 3).unwrap_or_default();
        self.git_info = git::load(&self.cwd).ok();
    }

    pub fn cursor(&self) -> usize {
        self.cursors[self.active as usize]
    }

    pub fn set_cursor(&mut self, v: usize) {
        self.cursors[self.active as usize] = v;
    }

    pub fn move_cursor(&mut self, delta: i32, max: usize) {
        if max == 0 {
            self.set_cursor(0);
            return;
        }
        let cur = self.cursor() as i32;
        let next = (cur + delta).clamp(0, (max as i32).saturating_sub(1));
        self.set_cursor(next as usize);
    }

    pub fn cycle_section(&mut self) {
        let idx = self.active as usize;
        let next = (idx + 1) % 3;
        self.active = Section::all()[next];
    }

    pub fn jump_section(&mut self, idx: u8) {
        if (idx as usize) < 3 {
            self.active = Section::all()[idx as usize];
        }
    }
}
