mod account_page;
mod homepage;
mod transaction_page;
pub use account_page::*;
pub use homepage::*;
pub use transaction_page::*;

use crate::data_layer::YnabApi;
use std::io;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

pub enum Message {
    Back,
    Quit,
    NewPage(Box<dyn Page>),
    Noop,
    Forward,
}

pub trait Page {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect);

    fn update(&mut self, data_gateway: &mut YnabApi) -> io::Result<Message>;

    fn name(&self) -> String;
}

pub fn noop() -> io::Result<Message> {
    Ok(Message::Noop)
}
