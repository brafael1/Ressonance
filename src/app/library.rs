use music_player::audio::get_track_duration;
use music_player::player::{find_audio_files, read_metadata};

use super::App;

impl App {
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
}
