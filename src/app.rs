use std::time::Duration;

use music_player::audio::{AudioState, get_track_duration};
use music_player::player::{find_audio_files, read_metadata, AppState, PlayerState};

pub struct App {
    pub state: AppState,
    pub audio: AudioState,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let (audio, audio_result) = AudioState::new();
        if let Err(e) = &audio_result {
            eprintln!("{}", e);
        }

        Self {
            state: AppState::new(),
            audio,
            should_quit: false,
        }
    }

    pub fn play_current(&mut self) {
        if let Some(track) = self.state.playlist.current_track() {
            let path = track.path.clone();
            let name = track.display_name();
            self.audio.play(path);
            self.state.player_state = PlayerState::Playing;
            self.state.elapsed = Duration::ZERO;
            self.state.set_status(format!("Playing: {}", name));
        }
    }

    pub fn toggle_playback(&mut self) {
        match self.state.player_state {
            PlayerState::Stopped => {
                self.play_current();
            }
            PlayerState::Playing => {
                self.audio.pause();
                self.state.player_state = PlayerState::Paused;
            }
            PlayerState::Paused => {
                self.audio.resume();
                self.state.player_state = PlayerState::Playing;
            }
        }
    }

    pub fn play_next(&mut self) {
        if self.state.playlist.is_empty() {
            return;
        }

        if let Some(track) = self.state.playlist.next() {
            let path = track.path.clone();
            let name = track.display_name();
            self.audio.play(path);
            self.state.player_state = PlayerState::Playing;
            self.state.elapsed = Duration::ZERO;
            self.state.set_status(format!("Playing: {}", name));
        }
    }

    pub fn play_previous(&mut self) {
        if self.state.playlist.is_empty() {
            return;
        }

        if let Some(track) = self.state.playlist.previous() {
            let path = track.path.clone();
            let name = track.display_name();
            self.audio.play(path);
            self.state.player_state = PlayerState::Playing;
            self.state.elapsed = Duration::ZERO;
            self.state.set_status(format!("Playing: {}", name));
        }
    }

    pub fn load_directory(&mut self, dir_str: &str) {
        let dir = std::path::PathBuf::from(dir_str);

        if !dir.exists() {
            self.state.set_error(format!("Directory not found: {}", dir_str));
            return;
        }

        if !dir.is_dir() {
            self.state.set_error(format!("Not a directory: {}", dir_str));
            return;
        }

        let files = find_audio_files(&dir);
        let count = files.len();

        if count == 0 {
            self.state
                .set_error(format!("No audio files found in {}", dir_str));
            return;
        }

        self.state.playlist.tracks.clear();
        self.state.playlist.current_index = 0;

        for path in files {
            let mut track = read_metadata(&path);
            track.duration = get_track_duration(&path);
            self.state.playlist.add_track(track);
        }

        self.state
            .set_status(format!("Added {} tracks from {}", count, dir_str));
    }

    pub fn reload_directory(&mut self) {
        if let Some(track) = self.state.playlist.current_track() {
            if let Some(parent) = track.path.parent() {
                let dir = parent.to_path_buf();
                let dir_str = dir.to_string_lossy().to_string();
                let files = find_audio_files(&dir);
                let count = files.len();

                self.state.playlist.tracks.clear();
                self.state.playlist.current_index = 0;

                for path in files {
                    let mut track = read_metadata(&path);
                    track.duration = get_track_duration(&path);
                    self.state.playlist.add_track(track);
                }

                self.state
                    .set_status(format!("Reloaded {} tracks from {}", count, dir_str));
            }
        }
    }

    pub fn change_volume(&mut self, delta: f32) {
        self.state.volume = (self.state.volume + delta).clamp(0.0, 1.0);
        self.audio.set_volume(self.state.volume);
    }

    pub fn update_audio_state(&mut self) {
        if self.state.player_state == PlayerState::Playing {
            let elapsed = *self.audio.elapsed.blocking_lock();
            self.state.elapsed = elapsed;

            if let Some(track) = self.state.playlist.current_track() {
                if let Some(duration) = track.duration {
                    if elapsed >= duration {
                        self.play_next();
                    }
                }
            }
        }
    }
}
