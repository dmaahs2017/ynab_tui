use tui::{backend::Backend, layout::Rect, style::*, terminal::Frame, widgets::*};

use crate::{data_layer::models::*, util::force_mut_ref};

use super::helpers::*;

#[derive(Clone)]
pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
    active: bool,
    title: String,
}

impl<T> std::ops::Index<usize> for StatefulList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: Clone> StatefulList<T> {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
            items: vec![],
            active: false,
            title: String::new(),
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) -> &mut Self {
        self.items = items;
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

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn get_selected(&self) -> Option<&T> {
        let i = self.state.selected()?;
        self.items.get(i)
    }

    /// Returns the new selected index or None if the list is empty
    pub fn select_next(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            self.unselect();
            return None;
        }

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
        Some(&self.items[i])
    }

    /// Returns the new selected index, or None if the list is empty
    pub fn select_prev(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            self.unselect();
            return None;
        }

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
        Some(&self.items[i])
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    fn ui<'a, F>(&'a self, line_to_str: F) -> List
    where
        F: Fn(T) -> String,
    {
        let block = if self.active {
            active_block().title(self.title.as_str())
        } else {
            block().title(self.title.as_str())
        };

        let budget_items = self
            .items
            .iter()
            .map(|item| list_item(line_to_str(item.clone())))
            .collect::<Vec<_>>();

        let budget_list = List::new(budget_items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        budget_list
    }
}

impl StatefulList<Budget> {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        f.render_stateful_widget(self.ui(|b| b.name), area, unsafe {
            force_mut_ref(&self.state)
        })
    }
}

impl StatefulList<Account> {
    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let widget = self.ui(|a| a.name).to_owned();
        f.render_stateful_widget(widget, area, unsafe { force_mut_ref(&self.state) })
    }
}
