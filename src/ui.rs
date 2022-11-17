use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Layout, Constraint, Direction, Rect},
    text::Spans,
    widgets::{
        Block, Borders, BorderType,
        Paragraph, Wrap
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(8), Constraint::Percentage(92)].as_ref())
        .split(f.size());
    draw_banner(f, app, chunks[0]);
    draw_play(f, app, chunks[1]);
}

fn draw_banner<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from("This is a music player writen by rust")
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(app.title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_play<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
}

