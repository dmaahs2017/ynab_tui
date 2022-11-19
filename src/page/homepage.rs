use super::*;
use crossterm::event::*;
use std::io;
use tui::layout::*;
use tui::style::*;
use tui::text::*;
use tui::widgets::*;

use crate::components::StatefulList;
use crate::data_layer::models::*;
use crate::data_layer::DataGateway;

pub struct Homepage {
    budgets: StatefulList<Budget>,
    transactions: Vec<Transaction>,
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

impl Page for Homepage {
    fn ui(&mut self, frame: &mut Frame<CrosstermBackend<io::Stdout>>, area: Rect) {
        let budget_items = self
            .budgets
            .items
            .iter()
            .map(|b| {
                let lines = vec![Spans::from(b.id.clone()), Spans::from(b.name.clone())];
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

        let transaction_items = self
            .transactions
            .iter()
            .map(|t| {
                let lines = vec![
                    t.date.clone().into(),
                    t.payee_name.into_spans(),
                    t.memo.into_spans(),
                    t.amount.to_string().into(),
                ];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect::<Vec<_>>();

        let transaction_list = List::new(transaction_items)
            .block(Block::default().borders(Borders::ALL).title("Budgets"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .direction(Direction::Horizontal)
            .split(area);

        if self.transactions.len() > 0 {
            frame.render_stateful_widget(budget_list, chunks[0], &mut self.budgets.state);
            frame.render_widget(transaction_list, chunks[1]);
        } else {
            frame.render_stateful_widget(budget_list, area, &mut self.budgets.state);
        }
    }

    fn update(&mut self) -> io::Result<Message> {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Message::Quit),
                KeyCode::Char('b') => return Ok(Message::Back),
                KeyCode::Char('k') => {
                    let dg = DataGateway::new();
                    let i = self.budgets.previous();
                    let b_id = &self.budgets.items[i].id;
                    self.transactions = dg.get_last_10_transactions(b_id);
                    return Ok(Message::Noop);
                }
                KeyCode::Char('j') => {
                    let dg = DataGateway::new();
                    let i = self.budgets.next();
                    let b_id = &self.budgets.items[i].id;
                    self.transactions = dg.get_last_10_transactions(b_id);
                    return Ok(Message::Noop);
                }
                KeyCode::Esc => {
                    self.budgets.unselect();
                    self.transactions.clear();
                    return Ok(Message::Noop);
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
