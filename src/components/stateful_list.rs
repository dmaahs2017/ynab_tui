use tui::widgets::*;

#[derive(Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    /// Returns the new selected index or None if the list is empty
    pub fn next(&mut self) -> Option<&T> {
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
    pub fn previous(&mut self) -> Option<&T> {
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
