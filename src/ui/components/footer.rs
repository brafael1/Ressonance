use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::player::{AppState, PlayerState};

fn accent_color() -> Color {
    Color::Cyan
}

pub fn render_footer(frame: &mut Frame, area: Rect, state: &AppState) {
    let status = match state.player_state {
        PlayerState::Playing => "▶ Playing",
        PlayerState::Paused => "❚❚ Paused",
        PlayerState::Stopped => "■ Stopped",
    };

    let status_line = Line::from(vec![
        Span::styled(
            status,
            Style::default()
                .fg(accent_color())
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(
            "[q]uit [space]play [n]ext [p]rev [s]earch [a]dd [↑↓]select [r]eload [enter]play",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let footer = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent_color())),
    );

    frame.render_widget(footer, area);
}

pub fn render_status_popup(frame: &mut Frame, state: &AppState) {
    if let Some(msg) = &state.status_message {
        let popup = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .title(" Status ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        let popup_area = centered_rect(50, 10, frame.area());
        frame.render_widget(popup, popup_area);
    }

    if let Some(err) = &state.error_message {
        let popup = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .title(" Error ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            );

        let popup_area = centered_rect(50, 10, frame.area());
        frame.render_widget(popup, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
