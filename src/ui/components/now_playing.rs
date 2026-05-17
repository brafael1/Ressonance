use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::player::{AppState, Track};
use crate::player::format_duration;

fn accent_color() -> Color {
    Color::Cyan
}

pub fn render_now_playing(frame: &mut Frame, area: Rect, state: &AppState) {
    let Some(track) = state.playlist.current_track() else {
        let np = Paragraph::new("No track playing")
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .title(" Now Playing ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(accent_color())),
            );

        frame.render_widget(np, area);
        return;
    };

    let progress = state.progress_percent();
    let progress_bar = render_progress_bar(progress, area.width as usize - 4);

    let lines = vec![
        Line::from(Span::styled(
            &track.title,
            Style::default()
                .fg(accent_color())
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            &track.artist,
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            &track.album,
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            progress_bar,
            Style::default().fg(accent_color()),
        )),
        Line::from(Span::styled(
            format!("{} / {}", state.format_elapsed(), track_duration_str(track)),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let np = Paragraph::new(lines).block(
        Block::default()
            .title(" Now Playing ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent_color())),
    );

    frame.render_widget(np, area);
}

fn render_progress_bar(percent: f64, width: usize) -> String {
    let filled = ((percent / 100.0) * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn track_duration_str(track: &Track) -> String {
    if let Some(d) = track.duration {
        format_duration(d)
    } else {
        "??:??".to_string()
    }
}
