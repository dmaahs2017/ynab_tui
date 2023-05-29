use super::*;
use crossterm::event::*;

use crate::{components::*, data_layer::*, util::*};
use std::{io, time::Duration};
use tui::layout::*;

#[derive(Clone)]
pub struct AccountsPage {
    budget: Budget,
    accounts: StatefulList<Account>,
    transactions: StatefulTable<Transaction>,
    command_pallete: CommandPallete,
    page_state: PageState,
}


impl AccountsPage {
    pub fn new(budget: Budget, api: &mut YnabApi) -> Self {
        let accounts = api.list_accounts(&budget.id).unwrap();
        let transactions = api.list_transactions(&budget.id, None).unwrap();
        Self {
            budget,
            accounts: StatefulList::with_items(accounts),
            transactions: StatefulTable::with_items(transactions),
            page_state: PageState::AccountSelect,
            command_pallete: Default::default(),
        }
    }


    fn edit_command(&mut self, event: Event, prev_state: PageState) -> io::Result<Message> {
        if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Char(c) => {
                        self.command_pallete.push(c);
                    }
                    KeyCode::Backspace => {
                        self.command_pallete.pop();
                    }
                    KeyCode::Enter => {
                        if prev_state == PageState::NavigateTable {
                            self.transactions.filter(&self.command_pallete)
                        }
                        self.page_state = prev_state
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
                    self.transactions.set_items(api.list_account_transactions(&self.budget.id, &account.id).unwrap());
                    self.transactions.filter(&self.command_pallete);
                }
                noop()
            }
            KeyCode::Char('j') => {
                if let Some(account) = self.accounts.select_next() {
                    self.transactions.set_items(api.list_account_transactions(&self.budget.id, &account.id).unwrap());
                    self.transactions.filter(&self.command_pallete);
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
                self.transactions.set_items(api.list_transactions(&self.budget.id, None).unwrap());
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
                self.page_state = PageState::AccountSelect;
                noop()
            }
            KeyCode::Char('/') => {
                self.command_pallete.clear();
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
    AccountSelect,
    EditCommand(Box<PageState>),
    NavigateTable,
    OverlayHelp,
    _ErrState(String),
}

impl PageState {
    fn is_edit(&self) -> bool {
        if let PageState::EditCommand(_) = self {
            return true
        }
        false
    }
}

impl Page for AccountsPage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let account_list = self.accounts.ui("Accounts", self.page_state == PageState::AccountSelect);
        let transactions_table = self.transactions.ui("Transactions", self.page_state == PageState::NavigateTable);
        let command_pallete = self.command_pallete.ui("Search", self.page_state.is_edit());


        if self.command_pallete.is_empty() && !self.page_state.is_edit() {
            let (master, stack) = master_stack_layout(1, 80, area);
            frame.render_stateful_widget(transactions_table, master, self.transactions.get_state_mut());
            frame.render_stateful_widget(account_list, stack[0], self.accounts.get_state_mut());
        } else {
            let (area, pallete_area) = split_vertical(90, area);
            let (master, stack) = master_stack_layout(1, 80, area);
            frame.render_stateful_widget(transactions_table, master, self.transactions.get_state_mut());
            frame.render_stateful_widget(account_list, stack[0], self.accounts.get_state_mut());
            frame.render_widget(command_pallete, pallete_area);
        }

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

        if let PageState::_ErrState(message) = &self.page_state {
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
            PageState::_ErrState(_) => {
                self.page_state = PageState::AccountSelect;
                noop()
            }
            PageState::OverlayHelp => {
                self.page_state = PageState::AccountSelect;
                noop()
            }
            PageState::EditCommand(prev_state) => self.edit_command(event, *prev_state),
            PageState::AccountSelect => self.select_account(event, api),
            PageState::NavigateTable => self.navigate_table(event),
        }
    }

    fn name(&self) -> String {
        self.accounts.get_selected().map(|a| a.name.clone()).unwrap_or("All Accounts".to_string())
    }
}

fn noop() -> io::Result<Message> {
    Ok(Message::Noop)
}
