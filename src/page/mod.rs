mod homepage;
mod page2;
pub use homepage::*;
pub use page2::*;

use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

pub enum Message {
    Back,
    Quit,
    NewPage(Box<dyn Page>),
    Noop,
    Forward,
}

pub trait Page {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect);

    fn update(&mut self) -> io::Result<Message>;

    fn name(&self) -> String;
}
