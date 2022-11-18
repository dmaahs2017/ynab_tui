use super::*;
use crossterm::event::*;
use std::io;
use tui::widgets::*;

pub struct Page2 {
    number: usize,
}

impl Page2 {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

impl Page for Page2 {
    fn ui(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let p = Paragraph::new(self.number.to_string())
            .block(Block::default().title("Page 2").borders(Borders::ALL));
        frame.render_widget(p, area);
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('n') => {
                    return Ok(Message::NewPage(Box::new(Homepage::new(
                        "Homepage spawned from counter page",
                    ))))
                }
                KeyCode::Char('f') => return Ok(Message::Forward),
                KeyCode::Char('+') => {
                    self.number += 1;
                    return Ok(Message::Noop);
                }
                _ => return Ok(Message::Noop),
            }
        }

        Ok(Message::Quit)
    }

    fn name(&self) -> String {
        String::from("Counter Page")
    }
}
