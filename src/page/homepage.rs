use super::*;
use crossterm::event::*;

use crate::{components::*, data_layer::*};
use std::{io, time::Duration};
use tui::layout::*;

pub struct Homepage {
    budgets: StatefulList<Budget>,
    page_state: PageState,
}

impl Homepage {
    pub fn new(api: &mut YnabApi) -> Self {
        let mut budgets = StatefulList::with_items(api.list_budgets().unwrap());
        budgets.select_next();
        Self {
            budgets,
            page_state: PageState::BudgetSelect,
        }
    }

    fn select_budget(&mut self, event: Event, api: &mut YnabApi) -> io::Result<Message> {
        #[rustfmt::skip]
        let key = if let Event::Key(key) = event { key } else { return noop(); };

        match key.code {
            KeyCode::Char('r') => {
                self.budgets = StatefulList::with_items(api.list_budgets().unwrap());
                noop()
            }
            KeyCode::Char('k') => {
                self.budgets.select_prev();
                noop()
            }
            KeyCode::Char('j') => {
                self.budgets.select_next();
                noop()
            }
            KeyCode::Enter => {
                if let Some(budget) = self.budgets.get_selected() {
                    return Ok(Message::NewPage(Box::new(BudgetPage::new(
                        budget.clone(),
                        api,
                    ))));
                };
                noop()
            }
            _ => noop(),
        }
    }
}

#[derive(PartialEq)]
enum PageState {
    BudgetSelect,
}

impl Page for Homepage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let budget_list = self.budgets.ui("Budgets", self.page_state == PageState::BudgetSelect);
        frame.render_stateful_widget(budget_list, area, self.budgets.get_state_mut());
    }

    fn update(&mut self, api: &mut YnabApi) -> io::Result<Message> {
        if let Ok(false) = poll(Duration::from_millis(200)) {
            return noop();
        }
        let event = read()?;

        // Global Keybinds, override any PageState Keybinds
        if let Event::Key(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => return Ok(Message::Quit),
                    _ => (),
                }
            }
        }

        match self.page_state {
            PageState::BudgetSelect => self.select_budget(event, api),
        }
    }

    fn name(&self) -> String {
        String::from("Homepage")
    }
}

fn noop() -> io::Result<Message> {
    Ok(Message::Noop)
}
