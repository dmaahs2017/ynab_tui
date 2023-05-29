use tui::widgets::*;

#[derive(Clone)]
pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

#[rustfmt::skip]
impl<T> Default for StatefulTable<T> { fn default() -> Self { Self::new() } }

impl<T> StatefulTable<T> {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::default(),
        }
    }
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: TableState::default(),
            items,
        }
    }

    pub fn select_next(&mut self) -> usize {
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
        i
    }

    pub fn select_prev(&mut self) -> usize {
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
        i
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
