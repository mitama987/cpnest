# cpnest

A GitHub Copilot CLI-aware terminal multiplexer. Windows-first. Sibling
project to [ccnest](https://github.com/mitama987/ccnest) (same shape, but
launches `copilot` instead of `claude` in every new pane).

**Docs site:** <https://mitama987.github.io/cpnest/>

## Status

v0.1 — Windows only. Rust + `ratatui` + `crossterm` + `portable-pty` + `vt100`.

## Install

Three options — all end up with a `cpnest` binary on your PATH, so you can
run it from any directory.

**Easiest (no Rust toolchain needed):**

Download the latest `cpnest.exe` from
[GitHub Releases](https://github.com/mitama987/cpnest/releases/latest) and
drop it anywhere on your PATH:

```powershell
# example: put it in ~/.local/bin
mkdir $HOME\.local\bin -Force
Move-Item .\cpnest.exe $HOME\.local\bin\
```

Make sure `$HOME\.local\bin` is on your PATH (or move to another dir that is).

**From source (rustup users):**

```sh
git clone https://github.com/mitama987/cpnest
cd cpnest
cargo install --path .
```

This drops `cpnest.exe` into `%USERPROFILE%\.cargo\bin\`, which rustup
already puts on PATH.

**Alternative (PowerShell, installs to `~/.local/bin`):**

```powershell
git clone https://github.com/mitama987/cpnest
cd cpnest
pwsh .\scripts\install.ps1
```

Override the destination with `$env:CPNEST_INSTALL_DIR` before running
`install.ps1`.

Requires a working `copilot` CLI on `PATH` (the interactive
[GitHub Copilot CLI](https://github.com/github/copilot-cli)). If the
binary is found at a non-standard path, set `CPNEST_COPILOT_BIN` to the
absolute path. If `copilot` is not found, panes fall back to the system
shell (`%ComSpec%` / `$SHELL`) so the multiplexer stays usable.

## Launch

From any directory:

```sh
cpnest                     # use the current directory as the initial pane cwd
cpnest path\to\project
```

## Default keybindings

| Key | Action |
|-----|--------|
| `Ctrl+D` | Split the focused pane vertically (new `copilot` to the right) |
| `Ctrl+E` | Split the focused pane horizontally (new `copilot` below) |
| `Ctrl+T` | Open a new tab with a fresh `copilot` pane |
| `Ctrl+W` | Close the focused pane (SIGTERM) |
| `Alt + ←` / `Alt + →` | Previous / next tab |
| `Ctrl+Tab` / `Ctrl+Shift+Tab` | Previous / next tab (alias) |
| `F2` | Rename the current tab (Enter to commit, Esc to cancel) |
| `Ctrl + ← → ↑ ↓` | Move pane focus |
| `Ctrl+F` | Toggle the left sidebar's file tree (opens it focused, closes when already on Files) |
| `Ctrl+B` | Toggle the entire left sidebar |
| `Ctrl+1` .. `Ctrl+3` | Jump to sidebar section (Files / Git / Panes) |
| `Tab` (sidebar focused) | Cycle sidebar section |
| `↑` / `↓` / `j` / `k` (sidebar focused) | Move selection cursor |
| `Enter` (on a file row) | Open the entry in `$EDITOR` (falls back to `code`) |
| `Ctrl+Q` | Quit |
| anything else | Sent to the focused pane |

### Ctrl+D and EOF

`Ctrl+D` is captured by the multiplexer before it reaches `copilot`, so
it will no longer send EOF. To exit a Copilot session, use the CLI's own
exit command or `Ctrl+W` to close the pane from the outside.

## Tabs

Each tab's title is initialized from the pane's current folder name
(e.g. `cpnest` when launched from the repo root). Press `F2` to rename
the active tab — type the new title, then `Enter` to commit or `Esc` to
cancel. The cursor (`▎`) is shown inline while editing.

## Sidebar

The left sidebar is always available (`Ctrl+B` toggles the whole sidebar,
`Ctrl+F` toggles the file tree view specifically). It has three
sections:

- **Files** — a tree of the focused pane's cwd (depth 3, honors
  `.gitignore`).
- **Git** — the current branch plus `M / S / ?` counts for the focused
  pane's cwd.
- **Panes** — a list of every pane in the current tab, marking the active
  one and whether Copilot is running.

The selection cursor is a solid white reversed block (`bg=white`,
`fg=black`, `REVERSED`), rendered over the full width of the row.

## Relationship to ccnest

This project is a sibling of [ccnest](https://github.com/mitama987/ccnest)
— the Claude Code version of the same idea. The core multiplexer code is
nearly identical; cpnest differs in:

- Launches `copilot` instead of `claude` in each pane
- Drops the per-pane Claude context-usage sidebar section (Copilot CLI
  does not expose equivalent per-session JSONL logs)
- Violet active-tool border (`#8957e5`) instead of orange

## Dev

```sh
cargo check
cargo test
cargo build --release   # target/release/cpnest.exe
```

## License

MIT
