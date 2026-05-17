mod command;
mod player;
mod state;
mod thread;

pub use state::AudioState;

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub fn get_track_duration(path: &PathBuf) -> Option<Duration> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("csv=p=0")
        .arg(path)
        .output()
        .ok()?;

    let duration_str = String::from_utf8(output.stdout).ok()?;
    let secs: f64 = duration_str.trim().parse().ok()?;
    Some(Duration::from_secs_f64(secs))
}

pub fn cleanup() {
    player::kill_ffplay_processes();
}
