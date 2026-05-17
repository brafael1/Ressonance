use std::time::Duration;

use super::format::format_duration;
use super::playlist::Playlist;
use super::state::PlayerState;

#[derive(Debug)]
pub struct AppState {
    pub playlist: Playlist,
    pub player_state: PlayerState,
    pub elapsed: Duration,
    pub volume: f32,
    pub search_query: String,
    pub is_searching: bool,
    pub needs_folder_dialog: bool,
    pub scroll_offset: usize,
    pub status_message: Option<String>,
    pub status_time: Option<std::time::Instant>,
    pub error_message: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            playlist: Playlist::new("Default".to_string()),
            player_state: PlayerState::Stopped,
            elapsed: Duration::ZERO,
            volume: 0.7,
            search_query: String::new(),
            is_searching: false,
            needs_folder_dialog: false,
            scroll_offset: 0,
            status_message: None,
            status_time: None,
            error_message: None,
        }
    }

    pub fn request_folder_dialog(&mut self) {
        self.needs_folder_dialog = true;
    }

    pub fn filtered_tracks(&self) -> Vec<(usize, &super::track::Track)> {
        if self.search_query.is_empty() {
            self.playlist
                .tracks
                .iter()
                .enumerate()
                .map(|(i, t)| (i, t))
                .collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.playlist
                .tracks
                .iter()
                .enumerate()
                .filter(|(_, t)| {
                    t.title.to_lowercase().contains(&query)
                        || t.artist.to_lowercase().contains(&query)
                        || t.album.to_lowercase().contains(&query)
                })
                .map(|(i, t)| (i, t))
                .collect()
        }
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some(msg);
        self.status_time = Some(std::time::Instant::now());
    }

    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
        self.status_time = Some(std::time::Instant::now());
    }

    pub fn clear_expired_status(&mut self) {
        if let Some(time) = self.status_time {
            if time.elapsed() > Duration::from_secs(3) {
                self.status_message = None;
                self.error_message = None;
                self.status_time = None;
            }
        }
    }

    pub fn format_elapsed(&self) -> String {
        format_duration(self.elapsed)
    }

    pub fn progress_percent(&self) -> f64 {
        if let Some(track) = self.playlist.current_track() {
            if let Some(duration) = track.duration {
                if duration.as_secs() > 0 {
                    return self.elapsed.as_secs_f64() / duration.as_secs_f64() * 100.0;
                }
            }
        }
        0.0
    }
}
