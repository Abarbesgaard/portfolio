use crate::cv::structs::cv_data::ContactInformation;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use uuid::Uuid;

impl ContactInformation {
    pub fn new(id: Uuid, email: String, phone_number: String) -> Self {
        Self {
            id,
            email,
            phone_number,
        }
    }

    fn c_info() -> ContactInformation {
        ContactInformation::new(
            Uuid::new_v4(),
            "Abarbesgaard@gmail.com".to_string(),
            "+45 21 76 26 15".to_string(),
        )
    }

    pub fn display_widget(frame: &mut Frame, area: Rect) {
        let title: String = String::from("Contact Information");
        let c_info = Self::c_info();
        let text = vec![
            Line::from(vec![
                Span::styled("Email: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", c_info.email)),
            ]),
            Line::from(vec![
                Span::styled("Age: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", c_info.phone_number)),
            ]),
        ];
        let block = Block::default().title(title).borders(Borders::ALL);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    // Standalone display method with no parameters
    pub fn display() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let area = f.area();
                Self::display_widget(f, area);
            })?;

            // wait for input
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }
}
