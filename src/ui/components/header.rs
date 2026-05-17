use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::player::AppState;

fn accent_color() -> Color {
    Color::Cyan
}

pub fn render_header(frame: &mut Frame, area: Rect, _state: &AppState) {
    let title_spans = vec![
        Span::styled("♪ ", Style::default().fg(accent_color()).add_modifier(Modifier::BOLD)),
        Span::styled(
            "Music Player",
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled("v0.1.0", Style::default().fg(Color::DarkGray)),
    ];

    let title = Paragraph::new(Line::from(title_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent_color())),
    );

    frame.render_widget(title, area);
}
