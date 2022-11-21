mod budget_page;
mod homepage;
pub use budget_page::*;
pub use homepage::*;

use std::io;
use tui::{backend::CrosstermBackend, layout::Rect, text::Spans, Frame};

trait TableWidget {
    fn to_table(&self) -> tui::widgets::Table;
}

trait IntoSpans {
    fn into_spans(&self) -> Spans;
}

impl IntoSpans for Option<String> {
    fn into_spans(&self) -> Spans {
        match self {
            None => Spans::from("(empty)"),
            Some(s) => Spans::from(s.clone()),
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

    fn update(&mut self) -> io::Result<Message>;

    fn name(&self) -> String;
}
