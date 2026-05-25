# Ressonance

A modern TUI music player built with [Ratatui](https://ratatui.rs/).

![License](https://img.shields.io/badge/license-GPL--3.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)

## Features

- **TUI Interface** — Terminal-based UI with playlist, now playing panel, and keyboard controls
- **Audio Playback** — Play, pause, resume, next/previous track navigation
- **Metadata Support** — Reads ID3 tags (title, artist, album) from audio files
- **Duration Detection** — Track durations via `ffprobe`
- **Search** — Filter tracks by title, artist, or album
- **Folder Dialog** — Native file picker to add music directories
- **Progress Bar** — Visual progress indicator with elapsed/total time
- **Status Popups** — Feedback messages for actions and errors
- **Supported Formats** — MP3, FLAC, WAV, OGG, M4A

## Requirements

- [Rust](https://www.rust-lang.org/) (edition 2021)
- [FFmpeg](https://ffmpeg.org/) — `ffplay` and `ffprobe` must be available in `$PATH`

## Installation

```bash
git clone https://github.com/brafael1/Ressonance.git
cd Ressonance
cargo build --release
```

The binary will be at `target/release/music-player`.

## Usage

```bash
cargo run --release
```

### Key Bindings

| Key | Action |
|---|---|
| `q` | Quit |
| `Space` | Play / Pause |
| `n` | Next track |
| `p` | Previous track |
| `s` | Search tracks |
| `a` | Add music folder |
| `r` | Reload current directory |
| `↑` / `↓` | Select track |
| `Enter` | Play selected track |
| `Esc` | Exit search mode |

## Architecture

```
src/
├── main.rs              # Entry point & event loop
├── lib.rs               # Library exports
├── app/
│   ├── mod.rs           # App struct (AppState + AudioState)
│   ├── playback.rs      # Play/pause/next/previous logic
│   ├── library.rs        # Directory loading & metadata scanning
│   └── update.rs         # Audio state sync & auto-advance
├── player/
│   ├── app_state.rs      # UI state (playlist, search, status)
│   ├── state.rs          # PlayerState enum
│   ├── track.rs          # Track data model
│   ├── playlist.rs       # Playlist management
│   ├── metadata.rs       # ID3 tag reading & file discovery
│   └── format.rs         # Duration formatting
├── audio/
│   ├── mod.rs             # Audio module + ffprobe duration helper
│   ├── state.rs           # AudioState (async command channel)
│   ├── command.rs          # AudioCommand enum
│   ├── player.rs           # ffplay process management (SIGSTOP/SIGCONT)
│   └── thread.rs           # Audio thread event loop
├── ui/
│   ├── mod.rs             # Top-level render
│   ├── layout.rs          # Layout computation
│   └── components/
│       ├── header.rs       # Title bar
│       ├── playlist.rs     # Track list with search
│       ├── now_playing.rs  # Now playing panel
│       ├── footer.rs       # Status bar & popups
│       ├── visualizer.rs   # Visualizer logic (WIP)
│       └── visualizer_render.rs
├── input.rs              # Keyboard input handler
└── terminal.rs           # Terminal setup/teardown
```

Audio playback uses `ffplay` in a dedicated thread with `SIGSTOP`/`SIGCONT` for pause/resume. Commands are sent via an unbounded Tokio channel.

## License

[GPL-3.0](./LICENSE)