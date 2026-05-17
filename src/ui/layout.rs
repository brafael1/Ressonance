use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub header: Rect,
    pub main: Rect,
    pub footer: Rect,
}

pub fn compute_layout(area: Rect) -> AppLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    AppLayout {
        header: chunks[0],
        main: chunks[1],
        footer: chunks[2],
    }
}

pub fn split_main(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    (chunks[0], chunks[1])
}
