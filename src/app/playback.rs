use std::time::Duration;

use music_player::player::PlayerState;

use super::App;

impl App {
    fn play_track(&mut self) {
        if let Some(track) = self.state.playlist.current_track() {
            let path = track.path.clone();
            let name = track.display_name();
            self.audio.play(path);
            self.state.player_state = PlayerState::Playing;
            self.state.elapsed = Duration::ZERO;
            self.state.set_status(format!("Playing: {}", name));
        }
    }

    pub fn play_current(&mut self) {
        self.play_track();
    }

    pub fn toggle_playback(&mut self) {
        match self.state.player_state {
            PlayerState::Stopped => {
                self.play_track();
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

    pub fn select_next(&mut self) {
        if !self.state.playlist.is_empty() {
            self.state.playlist.next();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.state.playlist.is_empty() {
            self.state.playlist.previous();
        }
    }
}
