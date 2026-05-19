use music_player::player::PlayerState;

use super::App;

impl App {
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
