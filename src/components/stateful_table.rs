use tui::{widgets::*, style::*, layout::Constraint};

use crate::{data_layer::models::Transaction, util::{milicent_to_dollars, force_mut_ref}};

use super::{block, selected_block};

#[derive(Clone)]
pub struct StatefulTable<T> {
    state: TableState,
    items: Vec<T>,
    filtered: Vec<T>,
}

#[rustfmt::skip]
impl<T> Default for StatefulTable<T> { fn default() -> Self { Self::new() } }

impl<T> StatefulTable<T> {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::new(),
            filtered: Vec::new(),
        }
    }
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: TableState::default(),
            items,
            filtered: Vec::new(),
        }
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

    pub fn get_state_mut(&self) -> &mut TableState {
        unsafe {
            force_mut_ref(&self.state)
        }
    }

    pub fn set_items(&mut self, transactions: Vec<T>) {
        self.filtered.clear();
        self.items = transactions;
        self.unselect();
    }

}

impl StatefulTable<Transaction> {
    pub fn ui<'a, 'b:'a>(&'a self, title: &'b str, selected: bool) -> Table {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let table: Vec<Row> = self.items
            .iter()
            .map(|transaction| {
                Row::new(vec![
                    Cell::from(transaction.payee_name.as_deref().unwrap_or_default()),
                    Cell::from(transaction.category_name.as_str()),
                    Cell::from(transaction.memo.as_deref().unwrap_or_default()),
                    Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                    Cell::from(transaction.date.as_str()),
                ])
            })
            .collect();

        let block = if selected {
            selected_block().title(title)
        } else {
            block().title(title)
        };

        Table::new(table)
            .header(Row::new(vec!["Payee", "Category", "Memo", "Amount", "Date"]))
            .block(block)
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(24),
                Constraint::Percentage(20),
                Constraint::Percentage(36),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ])
    }

    pub fn filter(&mut self, filter: &str) {
        //eprintln!("Filtering");
        //eprintln!("Len of items: {}, Len of filtered: {}", self.items.len(), self.filtered.len());

        self.items.extend(self.filtered.drain(..));

        //eprintln!("Len of items: {}, Len of filtered: {}", self.items.len(), self.filtered.len());
        let filtered = self.items.drain_filter(|t| {
            !format!("{}{}{}{}{}{}", 
                t.date, 
                t.amount,
                t.memo.clone().unwrap_or_default(),
                t.payee_name.clone().unwrap_or_default(),
                t.category_name,
                t.account_name
            ).to_lowercase().contains(filter)
        });
        self.filtered.extend(filtered);

        //eprintln!("Len of items: {}, Len of filtered: {}", self.items.len(), self.filtered.len());

        self.unselect();
    }
}
