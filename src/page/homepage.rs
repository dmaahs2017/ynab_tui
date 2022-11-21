use super::*;
use crossterm::event::*;
use std::io;
use tui::layout::*;
use tui::style::*;
use tui::text::*;
use tui::widgets::*;
use std::time::Duration;

use crate::util::*;
use crate::components::StatefulList;
use crate::data_layer::models::*;
use crate::data_layer::DataGateway;

pub struct Homepage {
    budgets: StatefulList<Budget>,
    transactions: Vec<Transaction>,
}

impl TableWidget for Vec<Transaction> {
    fn to_table(&self) -> Table {
        let table: Vec<Row> = self.iter().map(|transaction| {
            Row::new(vec![
                Cell::from(transaction.date.clone()),
                Cell::from(transaction.payee_name.clone().unwrap_or_default()),
                Cell::from(format!("${:.2}", milicent_to_dollars(transaction.amount))),
                Cell::from(transaction.memo.clone().unwrap_or_default())
            ])
            
        }).collect();

        Table::new(table)
            .header(Row::new(vec!["Date", "Payee", "Amount", "Memo"]))
            .block(Block::default().title("Transactions").borders(Borders::ALL))
            .widths(&[Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25),Constraint::Percentage(25)])
    }
}



impl Homepage {
    pub fn new() -> Self {
        let gateway = DataGateway::new();
        Self {
            budgets: StatefulList::with_items(gateway.get_budgets()),
            transactions: vec![],
        }
    }
}

enum InputMode {
    BudgetSelect,
    TransactionFilter,
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

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .direction(Direction::Horizontal)
            .split(area);

        if self.transactions.len() > 0 {
            frame.render_stateful_widget(budget_list, chunks[0], &mut self.budgets.state);
            frame.render_widget(transactions_table, chunks[1]);
        } else {
            frame.render_stateful_widget(budget_list, area, &mut self.budgets.state);
        }
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Ok(false) = poll(Duration::from_millis(200)) {
            return Ok(Message::Noop);
        }

        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('r') => {
                    let mut dg = DataGateway::new();
                    dg.refresh_db();
                    self.budgets = StatefulList::with_items(dg.get_budgets());
                    self.transactions.clear();
                    return Ok(Message::Noop);
                }
                KeyCode::Char('k') => {
                    if self.budgets.items.is_empty() {
                        return Ok(Message::Noop);
                    }

                    let dg = DataGateway::new();
                    let i = self.budgets.previous();
                    let b_id = &self.budgets.items[i].id;
                    self.transactions = dg.get_transactions(b_id);
                    return Ok(Message::Noop);
                }
                KeyCode::Char('j') => {
                    if self.budgets.items.is_empty() {
                        return Ok(Message::Noop);
                    }

                    let dg = DataGateway::new();
                    let i = self.budgets.next();
                    let b_id = &self.budgets.items[i].id;
                    self.transactions = dg.get_transactions(b_id);
                    return Ok(Message::Noop);
                }
                KeyCode::Esc => {
                    self.budgets.unselect();
                    self.transactions.clear();
                    return Ok(Message::Noop);
                },
                KeyCode::Enter => {
                    if let Some(selected_index) = self.budgets.state.selected() {
                        let budget = self.budgets.items[selected_index].clone();
                        return Ok(Message::NewPage(Box::new(BudgetPage::new(budget))))
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
