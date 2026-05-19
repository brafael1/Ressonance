mod library;
mod playback;
mod update;

use music_player::audio::AudioState;
use music_player::player::AppState;

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
}
