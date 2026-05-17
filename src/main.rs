mod app;
mod input;
mod terminal;

use crossterm::event::{self, Event, KeyEventKind};

use app::App;
use terminal::{restore_terminal, setup_terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    let mut terminal = setup_terminal()?;

    while !app.should_quit {
        terminal.draw(|frame| music_player::ui::render(frame, &app.state))?;

        if app.state.needs_folder_dialog {
            app.state.needs_folder_dialog = false;
            restore_terminal()?;

            let dir = rfd::FileDialog::new().pick_folder();
            terminal = setup_terminal()?;

            if let Some(dir) = dir {
                app.load_directory(&dir.to_string_lossy());
            }

            continue;
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }

        app.update_audio_state();
        app.state.clear_expired_status();
    }

    restore_terminal()?;
    music_player::audio::cleanup();
    Ok(())
}
