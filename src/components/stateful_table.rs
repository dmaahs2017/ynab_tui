use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::*,
    widgets::*,
    Frame,
};
use ynab_openapi::models::TransactionDetail;

use crate::util::{force_mut_ref, milicent_to_dollars};

use super::{active_block, block};

#[derive(Clone)]
pub struct StatefulTable<T> {
    state: TableState,
    items: Vec<T>,
    filtered: Vec<T>,
    title: String,
    active: bool,
}

#[rustfmt::skip]
impl<T> Default for StatefulTable<T> { fn default() -> Self { Self::new() } }

impl<T> StatefulTable<T> {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::new(),
            filtered: Vec::new(),
            title: String::new(),
            active: false,
        }
    }

    pub fn set_items(&mut self, transactions: Vec<T>) -> &mut Self {
        self.filtered.clear();
        self.items = transactions;
        self.unselect();
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn focus(&mut self) -> &mut Self {
        self.active = true;
        self
    }

    pub fn unfocus(&mut self) -> &mut Self {
        self.active = false;
        self
    }

    pub fn selected(&self) -> Option<&T> {
        let i = self.state.selected()?;
        self.items.get(i)
    }

    pub fn select_next(&mut self) -> usize {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        i
    }

    pub fn select_prev(&mut self) -> usize {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        i
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    fn ui<'a, F>(&'a self, to_cells: F) -> Table
    where
        F: Fn(&T) -> Vec<Cell<'a>>,
    {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let table: Vec<Row> = self
            .items
            .iter()
            .map(|items| Row::new(to_cells(items)))
            .collect();

        let block = if self.active {
            active_block().title(self.title.as_str())
        } else {
            block().title(self.title.as_str())
        };

        let table = Table::new(table)
            .header(Row::new([
                "Payee", "Category", "Memo", "Amount", "Date",
            ]))
            .block(block)
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(24),
                Constraint::Percentage(20),
                Constraint::Percentage(36),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]);
        table
    }
}

impl StatefulTable<TransactionDetail> {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let table = self.ui(|transaction| {
            vec![
                Cell::from(transaction.payee_name.clone().unwrap_or_default()),
                Cell::from(transaction.category_name.clone().unwrap_or_default()),
                Cell::from(transaction.memo.clone().unwrap_or_default()),
                Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                Cell::from(transaction.date.clone()),
            ]
        });

        f.render_stateful_widget(table, area, unsafe { force_mut_ref(&self.state) })
    }

    pub fn filter(&mut self, filter: &str) {
        self.items.append(&mut self.filtered);

        let filtered = self.items.drain_filter(|t| {
            !format!(
                "{}{}{}{}{}{}",
                t.date,
                t.amount,
                t.memo.clone().unwrap_or_default(),
                t.payee_name.clone().unwrap_or_default(),
                t.category_name.clone().unwrap_or_default(),
                t.account_name
            )
            .to_lowercase()
            .contains(filter)
        });
        self.filtered.extend(filtered);
        self.unselect();
    }
}
