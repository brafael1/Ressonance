use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::player::{AppState, PlayerState};

fn accent_color() -> Color {
    Color::Cyan
}

pub fn render_playlist(frame: &mut Frame, area: Rect, state: &AppState) {
    let tracks = state.filtered_tracks();

    if tracks.is_empty() {
        let msg = if state.search_query.is_empty() {
            "No tracks. Press 'a' to add music."
        } else {
            "No tracks match search."
        };

        let empty = Paragraph::new(msg)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(" Playlist ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(accent_color())),
            );

        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = tracks
        .iter()
        .map(|(actual_idx, track)| {
            let is_current = *actual_idx == state.playlist.current_index;
            let style = if is_current {
                Style::default()
                    .fg(accent_color())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if is_current {
                match state.player_state {
                    PlayerState::Playing => "▶ ",
                    PlayerState::Paused => "❚❚ ",
                    PlayerState::Stopped => "  ",
                }
            } else {
                "  "
            };

            let display = format!("{}{}", prefix, track.display_name());
            ListItem::new(Line::from(Span::styled(display, style)))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.playlist.current_index));

    let title = if state.is_searching {
        format!(" Search: {} ", state.search_query)
    } else {
        format!(" Playlist ({}) ", state.playlist.len())
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent_color())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut list_state);
}
