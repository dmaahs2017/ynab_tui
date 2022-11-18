mod page;
use crossterm::event::*;
use crossterm::terminal::*;
use crossterm::*;
use page::*;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::*;
use tui::Terminal;
use tui::widgets::*;

pub struct App {
    page_stack: Vec<Box<dyn Page>>,
    restore_stack: Vec<Box<dyn Page>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            page_stack: vec![Box::new(Homepage::new("Hello Homepage"))],
            restore_stack: vec![],
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            let names: Vec<String> = self.page_stack.iter().map(|p| p.name()).collect();
            let path = names.join(" -> ");
            if let Some(page) = self.page_stack.last_mut() {
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
                        .direction(Direction::Vertical)
                        .split(f.size());
                    let page_area = chunks[1];
                    let path_area = chunks[0];
                    let path = Paragraph::new(path);

                    f.render_widget(path, path_area);
                    page.ui(f, page_area);
                })?;

                let msg = page.update()?;
                match msg {
                    Message::Quit => break,
                    Message::Back => {
                        if self.page_stack.len() > 1 {
                            self.restore_stack.push(self.page_stack.pop().unwrap());
                        }
                    }
                    Message::Forward => {
                        if self.restore_stack.len() > 0 {
                            self.page_stack.push(self.restore_stack.pop().unwrap());
                        }
                    }
                    Message::NewPage(newpage) => {
                        self.page_stack.push(newpage);
                        self.restore_stack.clear();
                    }
                    Message::Noop => {}
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
