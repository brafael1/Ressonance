use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::player::AppState;

pub fn render_visualizer(frame: &mut Frame, area: Rect, state: &AppState) {
    let lines: Vec<Line> = state
        .visualizer
        .render(area.width as usize, area.height as usize)
        .into_iter()
        .map(|line| {
            Line::from(Span::styled(
                line,
                Style::default().fg(Color::Cyan),
            ))
        })
        .collect();

    let visualizer = Paragraph::new(lines);
    frame.render_widget(visualizer, area);
}
