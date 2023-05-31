use std::{io, time::Duration};

use crossterm::event::{poll, read, KeyEvent, Event, KeyModifiers, KeyCode};
use tui::{Frame, backend::CrosstermBackend, layout::{Rect, Layout, Constraint}, widgets::{*, self}, style::*};
use ynab_openapi::models::TransactionDetail;

use crate::{data_layer::YnabApi, components::StatefulList};

use super::*;

pub struct TransactionPage {
    transaction: StatefulList<String>
}

impl TransactionPage {
    pub fn new(t: TransactionDetail) -> Self {

        let mut transaction = StatefulList::new();
        let xs = vec![
            t.date,
            t.account_name,
            t.payee_name.unwrap_or_default(),
            t.category_name.unwrap_or_default(),
            t.memo.unwrap_or_default(),
            t.amount.to_string(),
        ];
        transaction.set_items(xs);
        Self { transaction }  
    }
}

impl Page for TransactionPage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        self.transaction.render(frame, area);
    }

    fn update(&mut self, data_gateway: &mut YnabApi) -> io::Result<Message> {
        if let Ok(false) = poll(Duration::from_millis(200)) {
            return noop();
        }
        let event = read()?;

        // Global Keybinds, override any PageState Keybinds
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('b') => return Ok(Message::Back),
                _ => (),
            }
        }

        noop()
    }

    fn name(&self) -> String {
        String::from("Inspect Transaction")
    }
}
