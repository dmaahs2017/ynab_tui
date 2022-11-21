use super::*;
use crossterm::event::*;
use std::io;
use tui::widgets::*;

use crate::data_layer::models::*;

pub struct BudgetPage {
    budget: Budget,
}

impl BudgetPage {
    pub fn new(budget: Budget) -> Self {
        Self { budget }
    }
}

impl Page for BudgetPage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let p = Paragraph::new(self.budget.name.to_string())
            .block(Block::default().title("Page 2").borders(Borders::ALL));
        frame.render_widget(p, area);
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('n') => {}
                KeyCode::Char('f') => return Ok(Message::Forward),
                _ => return Ok(Message::Noop),
            }
        }

        Ok(Message::Quit)
    }

    fn name(&self) -> String {
        self.budget.name.clone()
    }
}
