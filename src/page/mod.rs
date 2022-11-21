mod homepage;
mod budget_page;
pub use homepage::*;
pub use budget_page::*;

use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::text::Spans;
use tui::Frame;

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
