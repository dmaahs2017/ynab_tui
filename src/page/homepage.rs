use super::*;
use crossterm::event::*;

use crate::{components::*, data_layer::*, util::*};
use std::{io, time::Duration};
use tui::{layout::*, style::*, text::*, widgets::*};

pub struct Homepage {
    budgets: StatefulList<Budget>,
    transactions: Vec<Transaction>,
    search: String,
    page_state: PageState,
}

impl TableWidget for Vec<Transaction> {
    fn to_table(&self) -> Table {
        let table: Vec<Row> = self
            .iter()
            .map(|transaction| {
                Row::new(vec![
                    Cell::from(transaction.date.clone()),
                    Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                    Cell::from(transaction.memo.clone().unwrap_or_default()),
                ])
            })
            .collect();

        Table::new(table)
            .header(Row::new(vec!["Date", "Payee", "Amount", "Memo"]))
            .block(Block::default().title("Transactions").borders(Borders::ALL))
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
    }
}

impl Homepage {
    pub fn new() -> Self {
        let gateway = DataGateway::new();
        Self {
            budgets: StatefulList::with_items(gateway.get_budgets()),
            transactions: vec![],
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

    fn select_prev_budget(&mut self) {
        if self.budgets.items.is_empty() {
            return;
        }
        let prev = self.budgets.previous();
    }

    fn select_next_budget(&mut self) {
        if self.budgets.items.is_empty() {
            return;
        }
        let prev = self.budgets.next();
    }
}

enum PageState {
    BudgetSelect,
    EditSearch,
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

        let transactions_table = self.transactions.to_table();

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

        if self.transactions.len() > 0 {
            frame.render_stateful_widget(budget_list, bottom_left, &mut self.budgets.state);
            frame.render_widget(search_bar, top_left);
            frame.render_widget(transactions_table, right_area);
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

    fn update(&mut self) -> io::Result<Message> {
        if let Ok(false) = poll(Duration::from_millis(200)) {
            return Ok(Message::Noop);
        }

        if let Event::Key(key) = read()? {
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
                            let dg = DataGateway::new();
                            match dg.get_transactions_where(&b.id, &self.search) {
                                Ok(ts) => self.transactions = ts,
                                Err(e) => {
                                    self.page_state =
                                        PageState::ErrState(e.message.unwrap_or_default());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                return Ok(Message::Noop);
            }

            if let PageState::ErrState(_) = self.page_state {
                self.page_state = PageState::BudgetSelect;
                return Ok(Message::Noop);
            }

            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('r') => {
                    let mut dg = DataGateway::new();
                    if let Err(e) = dg.refresh_db() {
                        self.page_state = PageState::ErrState(e.message.unwrap_or_default());
                        return Ok(Message::Noop);
                    };
                    self.budgets = StatefulList::with_items(dg.get_budgets());
                    self.transactions.clear();
                    return Ok(Message::Noop);
                }
                KeyCode::Char('k') => {
                    self.select_prev_budget();
                    if let Some(b) = self.current_budget() {
                        let dg = DataGateway::new();
                        self.transactions = dg.get_transactions(&b.id);
                    }
                    return Ok(Message::Noop);
                }
                KeyCode::Char('j') => {
                    self.select_next_budget();
                    if let Some(b) = self.current_budget() {
                        let dg = DataGateway::new();
                        self.transactions = dg.get_transactions(&b.id);
                    }
                    return Ok(Message::Noop);
                }
                KeyCode::Char('/') => {
                    match self.page_state {
                        PageState::BudgetSelect => self.page_state = PageState::EditSearch,
                        _ => (),
                    }
                    return Ok(Message::Noop);
                }
                KeyCode::Esc => {
                    self.budgets.unselect();
                    self.transactions.clear();
                    return Ok(Message::Noop);
                }
                KeyCode::Enter => {
                    if let Some(selected_index) = self.budgets.state.selected() {
                        let budget = self.budgets.items[selected_index].clone();
                        return Ok(Message::NewPage(Box::new(BudgetPage::new(budget))));
                    };
                }
                _ => return Ok(Message::Noop),
            }
        }

        Ok(Message::Noop)
    }

    fn name(&self) -> String {
        String::from("Homepage")
    }
}
