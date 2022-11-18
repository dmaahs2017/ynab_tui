use super::*;
use crossterm::event::*;
use page2::Page2;
use std::io;
use tui::widgets::*;

pub struct Homepage {
    text: String,
}

impl Homepage {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Page for Homepage {
    fn ui(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let p = Paragraph::new(self.text.clone())
            .block(Block::default().title("Homepage").borders(Borders::ALL));
        frame.render_widget(p, area);
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('n') => return Ok(Message::NewPage(Box::new(Page2::new(0)))),
                KeyCode::Char('f') => return Ok(Message::Forward),
                _ => return Ok(Message::Noop),
            }
        }

        Ok(Message::Quit)
    }

    fn name(&self) -> String {
        String::from("Homepage")
    }
}
