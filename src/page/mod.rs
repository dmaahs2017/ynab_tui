mod budget_page;
mod homepage;
pub use budget_page::*;
pub use homepage::*;

use crate::{
    components::*,
    data_layer::{models::*, YnabApi},
    util::*,
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Rect, *},
    style::*,
    text::Spans,
    widgets::*,
    Frame,
};

trait TableWidget {
    fn to_table(&self) -> tui::widgets::Table;
}

impl TableWidget for Vec<Transaction> {
    fn to_table(&self) -> Table {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let table: Vec<Row> = self
            .iter()
            .map(|transaction| {
                Row::new(vec![
                    Cell::from(transaction.date.as_str()),
                    Cell::from(transaction.payee_name.as_deref().unwrap_or_default()),
                    Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                    Cell::from(transaction.memo.as_deref().unwrap_or_default()),
                ])
            })
            .collect();

        Table::new(table)
            .header(Row::new(vec!["Date", "Payee_Name", "Amount", "Memo"]))
            .block(block().title("Transactions"))
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
    }
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

    fn update(&mut self, data_gateway: &mut YnabApi) -> io::Result<Message>;

    fn name(&self) -> String;
}
