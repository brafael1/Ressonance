use ratatui::layout::Rect;
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
            "[q]uit [space]play [n]ext [p]rev [s]earch [a]dd [r]eload [enter]play",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let footer = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent_color())),
    );

    frame.render_widget(footer, area);

    if let Some(msg) = &state.status_message {
        let status_popup = Paragraph::new(msg.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .title(" Status ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        let popup_area = centered_rect(60, 3, area);
        frame.render_widget(status_popup, popup_area);
    }

    if let Some(err) = &state.error_message {
        let error_popup = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .title(" Error ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            );

        let popup_area = centered_rect(60, 3, area);
        frame.render_widget(error_popup, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
