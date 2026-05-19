pub mod components;
mod layout;

use crate::player::AppState;
use ratatui::Frame;

pub fn render(frame: &mut Frame, state: &AppState) {
    let layout = layout::compute_layout(frame.area());

    components::render_header(frame, layout.header, state);

    let (playlist_area, np_area) = layout::split_main(layout.main);
    components::render_playlist(frame, playlist_area, state);
    components::render_now_playing(frame, np_area, state);

    components::render_footer(frame, layout.footer, state);

    if state.player_state == crate::player::PlayerState::Playing {
        let vis_area = layout::compute_visualizer_area(frame.area());
        components::render_visualizer(frame, vis_area, state);
    }
}
