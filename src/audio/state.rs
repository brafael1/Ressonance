use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::sync::Mutex;

use super::command::AudioCommand;
use super::thread::audio_thread;

pub struct AudioState {
    pub is_playing: Arc<AtomicBool>,
    pub elapsed: Arc<Mutex<Duration>>,
    pub command_tx: mpsc::UnboundedSender<AudioCommand>,
}

impl AudioState {
    pub fn new() -> (Self, Result<(), String>) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let is_playing = Arc::new(AtomicBool::new(false));
        let elapsed = Arc::new(Mutex::new(Duration::ZERO));

        let is_playing_clone = is_playing.clone();
        let elapsed_clone = elapsed.clone();

        let result = std::thread::Builder::new()
            .name("audio-thread".to_string())
            .spawn(move || {
                audio_thread(command_rx, is_playing_clone, elapsed_clone);
            })
            .map(|_| ())
            .map_err(|e| format!("Failed to spawn audio thread: {}", e));

        (
            Self {
                is_playing,
                elapsed,
                command_tx,
            },
            result,
        )
    }

    pub fn play(&self, path: PathBuf) {
        let _ = self.command_tx.send(AudioCommand::Play(path));
    }

    pub fn pause(&self) {
        let _ = self.command_tx.send(AudioCommand::Pause);
    }

    pub fn resume(&self) {
        let _ = self.command_tx.send(AudioCommand::Resume);
    }

    pub fn stop(&self) {
        let _ = self.command_tx.send(AudioCommand::Stop);
    }

    pub fn set_volume(&self, _volume: f32) {}
}
