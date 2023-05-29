use tui::widgets::*;

use crate::{data_layer::models::*, util::force_mut_ref};

use super::helpers::*;

#[derive(Default, Clone)]
pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> std::ops::Index<usize> for StatefulList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn get_state(&self) -> &ListState {
        &self.state
    }

    pub fn get_state_mut(&self) -> &mut ListState {
        unsafe {
            force_mut_ref(&self.state)
        }
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

}

impl StatefulList<Budget> {
    pub fn ui<'a, 'b:'a>(&'a self, title: &'b str, selected: bool) -> List {
        let budget_items = self
            .items
            .iter()
            .map(|budget| list_item(&budget.name))
            .collect::<Vec<_>>();
        let mut budget_list = list(budget_items, title);
        if selected {
            budget_list = budget_list.block(selected_block().title(title))
        }
        budget_list
    }
}

impl StatefulList<Account> {
    pub fn ui<'a, 'b: 'a>(&'a self, title: &'b str, selected: bool) -> List<'a> {
        let account_list_items = self
            .items
            .iter()
            .map(|account| list_item(&account.name))
            .collect::<Vec<_>>();
        let mut account_list = list(account_list_items, title);
        if selected {
            account_list = account_list.block(selected_block().title(title))
        }
        account_list
    }
}
