use crate::util::*;
use tui::{
    backend::Backend,
    layout::*,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::*,
    Frame,
};

pub fn render_popup_message<B: Backend>(
    width: u16,
    height: u16,
    popup_area: Rect,
    alignment: Alignment,
    message: &str,
    frame: &mut Frame<B>,
) {
    let popup = Paragraph::new(message)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("Error"))
        .alignment(alignment);

    let popup_area = centered_rect(width, height, popup_area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup, popup_area);
}

pub fn block() -> Block<'static> {
    Block::default().borders(Borders::ALL)
}

pub fn active_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().bg(Color::Black))
}

pub fn list_item<'a>(item: String) -> ListItem<'a> {
    let lines = vec![Spans::from(item)];

    ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
}

pub fn list<'a>(items: Vec<ListItem<'a>>, title: &'a str) -> List<'a> {
    List::new(items)
        .block(block().title(title))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
