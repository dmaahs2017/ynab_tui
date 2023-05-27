use ynab_tui::{data_layer::DataGateway, page::*};

use crossterm::{event::*, terminal::*, *};
use std::io;
use tui::{backend::*, layout::*, widgets::*, *};


pub struct App {
    page_stack: Vec<Box<dyn Page>>,
    restore_stack: Vec<Box<dyn Page>>,
    data_gate: DataGateway,
}

#[rustfmt::skip]
impl Default for App { fn default() -> Self { Self::new() } }

impl App {
    pub fn new() -> Self {
        let data_gate = DataGateway::new();
        Self {
            page_stack: vec![Box::new(Homepage::new(&data_gate))],
            restore_stack: vec![],
            data_gate,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut terminal = setup_terminal()?;

        loop {
            let names: Vec<String> = self.page_stack.iter().map(|p| p.name()).collect();
            let path = names.join(" -> ");

            let Some(page) = self.page_stack.last_mut() else { continue };

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

            let msg = page.update(&mut self.data_gate)?;

            match msg {
                Message::Quit => break,
                Message::Back => {
                    if self.page_stack.len() > 1 {
                        self.restore_stack.push(self.page_stack.pop().unwrap());
                    }
                }
                Message::Forward => {
                    if self.restore_stack.is_empty() {
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

        // restore terminal
        restore_terminal(terminal)?;

        Ok(())
    }
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    App::new().run()
}
