use super::*;
use crossterm::event::*;

use crate::{components::*, data_layer::*, util::*};
use std::{io, time::Duration};
use tui::{layout::*, style::*, widgets::*};

#[derive(Clone)]
pub struct BudgetPage {
    budget: Budget,
    accounts: StatefulList<Account>,
    transactions: StatefulTable<Transaction>,
    search: String,
    page_state: PageState,
}


impl BudgetPage {
    pub fn new(budget: Budget, api: &mut YnabApi) -> Self {
        let accounts = api.list_accounts(&budget.id).unwrap();
        let transactions = api.list_transactions(&budget.id, None).unwrap();
        Self {
            budget,
            accounts: StatefulList::with_items(accounts),
            transactions: StatefulTable::with_items(transactions),
            page_state: PageState::BudgetSelect,
            search: String::new(),
        }
    }

    fn current_account(&self) -> Option<&Account> {
        self.accounts
            .state
            .selected()
            .map(|i| &self.accounts.items[i])
    }

    fn edit_search(&mut self, event: Event, _prev_state: PageState, api: &mut YnabApi) -> io::Result<Message> {
        if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char(c) => {
                        self.search.push(c);
                    }
                    KeyCode::Backspace => {
                        self.search.pop();
                    }
                    KeyCode::Enter => {
                        self.page_state = PageState::BudgetSelect;
                        if let Some(account) = self.current_account() {
                            match api.list_account_transactions(&account.id, &self.search) {
                                Ok(ts) => {
                                    // TODO: Create DDL for filtering
                                    self.transactions.items = ts;
                                    self.transactions.unselect();
                                }
                                Err(e) => {
                                    self.page_state =
                                        PageState::ErrState(e.to_string());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                return noop();
        }
        noop()
    }

    fn select_account(&mut self, event: Event, api: &mut YnabApi) -> io::Result<Message> {
        let key = if let Event::Key(key) = event {
            key
        } else {
            return noop();
        };

        match key.code {
            KeyCode::Char('b') => Ok(Message::Back),
            KeyCode::Char('r') => {
                self.accounts = StatefulList::with_items(api.list_accounts(&self.budget.id).unwrap());
                noop()
            }
            KeyCode::Char('k') => {
                if let Some(account) = self.accounts.select_prev() {
                    self.transactions.items = api.list_account_transactions(&self.budget.id, &account.id).unwrap();
                    self.transactions.unselect()
                }
                noop()
            }
            KeyCode::Char('j') => {
                if let Some(account) = self.accounts.select_next() {
                    self.transactions.items = api.list_account_transactions(&self.budget.id, &account.id).unwrap();
                    self.transactions.unselect()
                }
                noop()
            }
            KeyCode::Char('/') => {
                self.switch_to_edit_state();
                noop()
            }
            KeyCode::Char('l') => {
                self.page_state = PageState::NavigateTable;
                noop()
            }
            KeyCode::Esc => {
                self.accounts.unselect();
                self.transactions.items = api.list_transactions(&self.budget.id, None).unwrap();
                self.transactions.unselect();
                noop()
            }
            KeyCode::Enter => {
                noop()
            }
            _ => noop(),
        }
    }

    fn navigate_table(&mut self, event: Event) -> io::Result<Message> {
        let key = if let Event::Key(key) = event {
            key
        } else {
            return noop();
        };

        match key.code {
            KeyCode::Char('j') => {
                self.transactions.select_next();
                noop()
            }
            KeyCode::Char('k') => {
                self.transactions.select_prev();
                noop()
            }
            KeyCode::Char('h') => {
                self.page_state = PageState::BudgetSelect;
                noop()
            }
            KeyCode::Char('/') => {
                self.switch_to_edit_state();
                noop()
            }
            _ => noop(),
        }
    }

    fn switch_to_edit_state(&mut self) {
        let prev = Box::new(self.page_state.clone());
        self.page_state = PageState::EditCommand(prev);
    }
}

#[derive(PartialEq, Clone)]
enum PageState {
    BudgetSelect,
    EditCommand(Box<PageState>),
    NavigateTable,
    OverlayHelp,
    ErrState(String),
}

impl Page for BudgetPage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let budget_items = self
            .accounts
            .items
            .iter()
            .map(|b| list_item(&b.name))
            .collect::<Vec<_>>();
        let mut budget_list = list(budget_items, "Accounts");
        if self.page_state == PageState::BudgetSelect {
            budget_list = budget_list.block(selected_block().title("Accounts"))
        }

        let mut command_pallete = Paragraph::new(self.search.as_str())
            .style(Style::default().add_modifier(Modifier::RAPID_BLINK))
            .block(block().title("SQL Filter"))
            .wrap(Wrap { trim: false });
        if let PageState::EditCommand(_) = self.page_state {
            command_pallete = command_pallete.block(selected_block().title("Command"));
        }

        let mut transactions_table = self.transactions.items.to_table();
        if self.page_state == PageState::NavigateTable {
            transactions_table = transactions_table.block(selected_block().title("Transactions"))
        }

        let (area, pallete_area) = split_vertical(90, area);
        let (master, stack) = master_stack_layout(1, 80, area);
        frame.render_stateful_widget(transactions_table, master, &mut self.transactions.state);
        frame.render_stateful_widget(budget_list, stack[0], &mut self.accounts.state);

        frame.render_widget(command_pallete, pallete_area);

        if let PageState::OverlayHelp = self.page_state {
            let help_text = vec![
                "?         Open Help",
                "k         Move Up",
                "j         Move Down",
                "l         Move Right",
                "h         Move Left",
                "ctrl-c    Quit",
                "/         Edit Filter Query",
            ]
            .join("\n");
            render_popup_message(30, 70, area, Alignment::Left, &help_text, frame);
        }

        if let PageState::ErrState(message) = &self.page_state {
            render_popup_message(30, 30, area, Alignment::Center, message, frame)
        }
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
                    KeyCode::Char('h') => {
                        self.page_state = PageState::OverlayHelp;
                        return noop();
                    }
                    _ => (),
                }
            }
        }

        match self.page_state.clone() {
            PageState::ErrState(_) => {
                self.page_state = PageState::BudgetSelect;
                noop()
            }
            PageState::OverlayHelp => {
                self.page_state = PageState::BudgetSelect;
                noop()
            }
            PageState::EditCommand(prev_state) => self.edit_search(event, *prev_state, api),
            PageState::BudgetSelect => self.select_account(event, api),
            PageState::NavigateTable => self.navigate_table(event),
        }
    }

    fn name(&self) -> String {
        String::from("BudgetPage")
    }
}

fn noop() -> io::Result<Message> {
    Ok(Message::Noop)
}
