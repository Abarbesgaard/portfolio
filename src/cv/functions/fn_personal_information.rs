use crate::cv::structs::cv_data::PersonalInformation;
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

impl PersonalInformation {
    pub fn new(
        id: Uuid,
        first_name: String,
        last_name: String,
        title: String,
        age: u8,
        tag_line: String,
        description: String,
        short_description: String,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            title,
            age,
            tag_line,
            description,
            short_description,
        }
    }

    fn p_info() -> PersonalInformation {
        PersonalInformation::new(
            Uuid::new_v4(),
            "Andreas".to_string(),
            "Barbesgaard".to_string(),
            "Software Developer".to_string(),
            36,
            "best dev ever".to_string(),
            "test description".to_string(),
            "test short description".to_string(),
        )
    }

    pub fn display_widget(frame: &mut Frame, area: Rect) {
        let p_info = Self::p_info();
        let text = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} {}", p_info.first_name, p_info.last_name)),
            ]),
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", p_info.title)),
            ]),
            Line::from(vec![
                Span::styled("Age: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", p_info.age)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Tag: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&p_info.tag_line),
            ]),
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Yellow)),
                Span::raw(&p_info.description),
            ]),
            Line::from(vec![
                Span::styled("Short Description: ", Style::default().fg(Color::Green)),
                Span::raw(&p_info.short_description),
            ]),
        ];
        let block = Block::default().title("Profile").borders(Borders::ALL);
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
