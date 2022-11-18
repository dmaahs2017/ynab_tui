use super::*;
use crossterm::event::*;
use std::io;
pub struct Homepage {
    text: String,
}

impl Page for Homepage {
    fn ui<B: Backend>(&self, frame: &mut Frame<B>) {
        todo!()
    }

    fn update(&mut self) -> std::io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('n') => {

                }
            }
        }

        Ok(Message::Quit)
    }
}
