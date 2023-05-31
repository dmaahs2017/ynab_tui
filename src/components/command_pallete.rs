use std::ops::{Deref, DerefMut};

use tui::{style::*, widgets::*};

use super::{active_block, block};

#[derive(Default, Clone)]
pub struct CommandPallete {
    command: String,
}

impl Deref for CommandPallete {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl DerefMut for CommandPallete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command
    }
}

impl CommandPallete {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui<'a, 'b: 'a>(&'a self, title: &'b str, selected: bool) -> Paragraph {
        let block = if selected {
            active_block().title(title)
        } else {
            block().title(title)
        };

        Paragraph::new(self.command.as_str())
            .style(Style::default().add_modifier(Modifier::RAPID_BLINK))
            .block(block)
            .wrap(Wrap { trim: false })
    }
}
