use super::*;
use crossterm::event::*;

use crate::{components::*, data_layer::*, util::*};
use std::{io, time::Duration};
use tui::{layout::*, style::*, text::*, widgets::*};

pub struct Homepage {
    budgets: StatefulList<Budget>,
    transactions: StatefulTable<Transaction>,
    search: String,
    page_state: PageState,
}

impl TableWidget for Vec<Transaction> {
    fn to_table(&self) -> Table {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let table: Vec<Row> = self
            .iter()
            .map(|transaction| {
                Row::new(vec![
                    Cell::from(transaction.date.clone()),
                    Cell::from(transaction.payee_name.clone().unwrap_or_default()),
                    Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                    Cell::from(transaction.memo.clone().unwrap_or_default()),
                ])
            })
            .collect();

        Table::new(table)
            .header(Row::new(vec!["Date", "Payee_Name", "Amount", "Memo"]))
            .block(Block::default().title("Transactions").borders(Borders::ALL))
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
    }
}

impl Homepage {
    pub fn new(data_gate: &DataGateway) -> Self {
        Self {
            budgets: StatefulList::with_items(data_gate.get_budgets()),
            transactions: StatefulTable::new(),
            page_state: PageState::BudgetSelect,
            search: String::new(),
        }
    }

    fn current_budget(&self) -> Option<&Budget> {
        self.budgets
            .state
            .selected()
            .map(|i| &self.budgets.items[i])
    }

    fn edit_search(&mut self, event: Event, dg: &mut DataGateway) -> io::Result<Message> {
        if let Event::Key(key) = event {
            if let PageState::EditSearch = self.page_state {
                match key.code {
                    KeyCode::Char(c) => {
                        self.search.push(c);
                    }
                    KeyCode::Backspace => {
                        self.search.pop();
                    }
                    KeyCode::Enter => {
                        self.page_state = PageState::BudgetSelect;
                        if let Some(b) = self.current_budget() {
                            match dg.get_transactions_where(&b.id, &self.search) {
                                Ok(ts) => { 
                                    self.transactions.items = ts ;
                                    self.transactions.unselect();
                                },
                                Err(e) => {
                                    self.page_state =
                                        PageState::ErrState(e.message.unwrap_or_default());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                return noop()
            }
        }
        noop()
    }

    fn select_budget(&mut self, event: Event, dg: &mut DataGateway) -> io::Result<Message> {
        let key = if let Event::Key(key) = event { key } else { return noop() };

        match key.code {
            KeyCode::Char('b') => Ok(Message::Back),
            KeyCode::Char('r') => {
                if let Err(e) = dg.refresh_db() {
                    self.page_state = PageState::ErrState(e.message.unwrap_or_default());
                    return noop()
                };
                self.budgets = StatefulList::with_items(dg.get_budgets());
                self.transactions.items.clear();
                self.transactions.unselect();
                noop()
            }
            KeyCode::Char('k') => {
                if let Some(b) = self.budgets.previous() {
                    self.transactions.items = dg.get_transactions(&b.id);
                    self.transactions.unselect()
                }
                return noop()
            }
            KeyCode::Char('j') => {
                if let Some(b) = self.budgets.next() {
                    self.transactions.items = dg.get_transactions(&b.id);
                    self.transactions.unselect()
                }
                return noop()
            }
            KeyCode::Char('/') => {
                self.page_state = PageState::EditSearch;
                noop()
            }
            KeyCode::Char('l') => {
                self.page_state = PageState::NavigateTable;
                noop()
            }
            KeyCode::Esc => {
                self.budgets.unselect();
                self.transactions.items.clear();
                self.transactions.unselect();
                noop()
            }
            KeyCode::Enter => {
                if let Some(selected_index) = self.budgets.state.selected() {
                    let budget = self.budgets.items[selected_index].clone();
                    return Ok(Message::NewPage(Box::new(BudgetPage::new(budget))));
                };
                noop()
            }
            _ => noop()
        }
    }

    fn navigate_table(&mut self, event: Event) -> io::Result<Message> {
        let key = if let Event::Key(key) = event { key } else { return noop() };

        match key.code {
            KeyCode::Char('j') => {
                self.transactions.next();
                noop()
            },
            KeyCode::Char('k') => {
                self.transactions.previous();
                noop()
            },
            KeyCode::Char('h') => {
                self.page_state = PageState::BudgetSelect;
                noop()
            }
            _ => noop(),
        }
    }
}

enum PageState {
    BudgetSelect,
    EditSearch,
    NavigateTable,
    ErrState(String),
}

impl Page for Homepage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        frame.render_widget(Clear, area);
        let budget_items = self
            .budgets
            .items
            .iter()
            .map(|b| {
                let lines = vec![Spans::from(b.name.clone())];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect::<Vec<_>>();

        let budget_list = List::new(budget_items)
            .block(Block::default().borders(Borders::ALL).title("Budgets"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");


        let search_bar = Paragraph::new(self.search.clone())
            .style(Style::default().add_modifier(Modifier::RAPID_BLINK))
            .block(Block::default().borders(Borders::ALL).title("SQL Filter"))
            .wrap(Wrap { trim: false });

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .direction(Direction::Horizontal)
            .split(area);
        let left_area = chunks[0];
        let right_area = chunks[1];

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Vertical)
            .split(left_area);
        let top_left = chunks[0];
        let bottom_left = chunks[1];

        if self.transactions.items.len() > 0 {
            let transactions_table = self.transactions.items.to_table();

            frame.render_stateful_widget(budget_list, bottom_left, &mut self.budgets.state);
            frame.render_widget(search_bar, top_left);
            frame.render_stateful_widget(transactions_table, right_area, &mut self.transactions.state);
        } else {
            frame.render_stateful_widget(budget_list, area, &mut self.budgets.state);
        }

        if let PageState::ErrState(message) = &self.page_state {
            let err_popup = Paragraph::new(message.clone())
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .alignment(Alignment::Center);

            let popup_area = centered_rect(30, 30, area);
            frame.render_widget(Clear, popup_area);
            frame.render_widget(err_popup, popup_area);
        }
    }

    fn update(&mut self, dg: &mut DataGateway) -> io::Result<Message> {
        if let Ok(false) = poll(Duration::from_millis(200)) {
            return noop()
        }
        let event = read()?;

        // Global Keybinds, override any PageState Keybinds
        if let Event::Key(key) = event { 
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => { return Ok(Message::Quit) },
                    _ => (),
                }
            }
        }

        match self.page_state {
            PageState::ErrState(_) => {
                self.page_state = PageState::BudgetSelect;
                noop()
            },
            PageState::EditSearch => self.edit_search(event, dg),
            PageState::BudgetSelect => self.select_budget(event, dg),
            PageState::NavigateTable => self.navigate_table(event),
        }
    }

    fn name(&self) -> String {
        String::from("Homepage")
    }
}

fn noop() -> io::Result<Message> {
    Ok(Message::Noop)
}
