use crossterm::event::KeyCode;

use super::App;

impl App {
    pub fn handle_key(&mut self, key: KeyCode) {
        if self.state.is_searching {
            match key {
                KeyCode::Esc | KeyCode::Enter => {
                    self.state.is_searching = false;
                }
                KeyCode::Char(c) => {
                    self.state.search_query.push(c);
                }
                KeyCode::Backspace => {
                    self.state.search_query.pop();
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') => {
                self.toggle_playback();
            }
            KeyCode::Char('n') => {
                self.play_next();
            }
            KeyCode::Char('p') => {
                self.play_previous();
            }
            KeyCode::Char('s') => {
                self.state.search_query.clear();
                self.state.is_searching = true;
            }
            KeyCode::Char('a') => {
                self.state.request_folder_dialog();
            }
            KeyCode::Char('r') => {
                self.reload_directory();
            }
            KeyCode::Up => {
                self.select_previous();
            }
            KeyCode::Down => {
                self.select_next();
            }
            KeyCode::Enter => {
                if !self.state.playlist.is_empty() {
                    self.play_current();
                }
            }
            _ => {}
        }
    }
}
