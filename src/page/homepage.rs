use super::*;
use crossterm::event::*;
use page2::Page2;
use std::io;
use tui::widgets::*;
use tui::style::*;
use tui::text::*;

use crate::components::StatefulList;
use crate::data_layer::DataGateway;
use crate::data_layer::models::Budget;

pub struct Homepage {
    budgets: StatefulList<Budget>,
}

impl Homepage {
    pub fn new() -> Self {
        let gateway = DataGateway::new();
        Self {
            budgets: StatefulList::with_items(gateway.get_budgets()),
        }
    }
}

impl Page for Homepage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let items = self.budgets.items.iter().map(|b| {
            let lines = vec![Spans::from(b.id.clone()), Spans::from(b.name.clone())];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        }).collect::<Vec<_>>();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Budgets"))
            .highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(items, area, &mut self.budgets.state);
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('k') => { self.budgets.previous(); return Ok(Message::Noop) },
                KeyCode::Char('j') => { self.budgets.next(); return Ok(Message::Noop) },
                KeyCode::Esc => {self.budgets.unselect(); return Ok(Message::Noop) }
                _ => return Ok(Message::Noop),
            }
        }

        Ok(Message::Noop)
    }

    fn name(&self) -> String {
        String::from("Homepage")
    }
}
