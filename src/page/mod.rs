mod budget_page;
mod homepage;
pub use budget_page::*;
pub use homepage::*;

use crate::data_layer::DataGateway;

use std::io;
use tui::{backend::CrosstermBackend, layout::Rect, text::Spans, Frame};

trait TableWidget {
    fn to_table(&self) -> tui::widgets::Table;
}

trait ToSpan {
    fn to_span(&self) -> Spans;
}

impl ToSpan for Option<String> {
    fn to_span(&self) -> Spans {
        match self {
            None => Spans::from("(empty)"),
            Some(s) => Spans::from(s.as_str()),
        }
    }
}

pub enum Message {
    Back,
    Quit,
    NewPage(Box<dyn Page>),
    Noop,
    Forward,
}

pub trait Page {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect);

    fn update(&mut self, data_gateway: &mut DataGateway) -> io::Result<Message>;

    fn name(&self) -> String;
}
