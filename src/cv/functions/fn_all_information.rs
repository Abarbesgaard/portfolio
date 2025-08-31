use crate::cv::structs::cv_data::{AllInformation, ContactInformation, PersonalInformation};
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

impl AllInformation {
    pub fn new(
        id: Uuid,
        personal_information: PersonalInformation,
        contact_information: ContactInformation,
    ) -> Self {
        Self {
            id,
            personal_information,
            contact_information,
        }
    }

    fn all_info() -> AllInformation {
        AllInformation::new(
            Uuid::new_v4(),
            PersonalInformation::new(
                Uuid::new_v4(),
                "Andreas".to_string(),
                "Barbesgaard".to_string(),
                "Software Developer".to_string(),
                36,
                "best dev".to_string(),
                "description".to_string(),
                "description_short".to_string(),
            ),
            ContactInformation::new(
                Uuid::new_v4(),
                "Abarbesgaard@gmail.com".to_string(),
                "+4521762615".to_string(),
            ),
        )
    }

    fn quit_string() -> Line<'static> {
        Line::from(vec![Span::styled(
            "Press q to quit",
            Style::default().add_modifier(Modifier::ITALIC),
        )])
    }

    // Widget for personal information
    fn display_personal_widget(frame: &mut Frame, area: Rect, personal_info: &PersonalInformation) {
        let title = String::from("Personal Information");
        let text = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!(
                    "{} {}",
                    personal_info.first_name, personal_info.last_name
                )),
            ]),
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", personal_info.title)),
            ]),
            Line::from(vec![
                Span::styled("Age: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", personal_info.age)),
            ]),
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", personal_info.title)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Description: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{}", personal_info.description)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Short Description: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{}", personal_info.short_description)),
            ]),
        ];

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    // Widget for contact information
    fn display_contact_widget(frame: &mut Frame, area: Rect, contact_info: &ContactInformation) {
        let title = String::from("Contact Information");
        let text = vec![
            Line::from(vec![
                Span::styled("Email: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", contact_info.email)),
            ]),
            Line::from(vec![
                Span::styled("Phone: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", contact_info.phone_number)),
            ]),
        ];

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    // Widget for instructions
    fn display_instructions_widget(frame: &mut Frame, area: Rect) {
        let title = String::from("Instructions");
        let text = vec![
            Self::quit_string(),
            Line::from(vec![Span::styled(
                "Use arrow keys to navigate (if implemented)",
                Style::default().fg(Color::Yellow),
            )]),
        ];

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    // Main display widget that creates separate windows
    pub fn display_widget(frame: &mut Frame, area: Rect) {
        let all_info = Self::all_info();

        // Split the main area into sections
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60), // Top section for main content
                Constraint::Percentage(40), // Bottom section for instructions
            ])
            .split(area);

        // Split the top section horizontally for personal and contact info
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left half for personal info
                Constraint::Percentage(50), // Right half for contact info
            ])
            .split(main_chunks[0]);

        // Render each widget in its own area
        Self::display_personal_widget(frame, top_chunks[0], &all_info.personal_information);
        Self::display_contact_widget(frame, top_chunks[1], &all_info.contact_information);
        Self::display_instructions_widget(frame, main_chunks[1]);
    }

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

            // Wait for input
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
}

