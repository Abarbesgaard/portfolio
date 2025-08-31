use crate::cv::structs::cv_data::{
    AllInformation, ContactInformation, Experience, PersonalInformation, Skill,
};
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
        experience_list: Vec<Experience>,
    ) -> Self {
        Self {
            id,
            personal_information,
            contact_information,
            experience_list,
        }
    }

    fn all_info() -> AllInformation {
        let skill_list = vec![Skill::new(
            Uuid::new_v4(),
            "Rust".to_string(),
            "My experience with rust".to_string(),
        )];
        let experience_list = vec![Experience::new(
            Uuid::new_v4(),
            "test dev title".to_string(),
            "very short description".to_string(),
            "04-03-2016".to_string(),
            "04-03-2018".to_string(),
            skill_list,
        )];

        AllInformation::new(
            Uuid::new_v4(),
            PersonalInformation::new(
                Uuid::new_v4(),
                "Andreas".to_string(),
                "Barbesgaard".to_string(),
                "Software Developer".to_string(),
                36,
                "best dev".to_string(),
               "Software developer skilled in .NET (C#), React, and TypeScript. Combining technical expertise with a background in pedagogy, I create solutions that are both functional and human-centered.".to_string(),
            ),
            ContactInformation::new(
                Uuid::new_v4(),
                "Abarbesgaard@gmail.com".to_string(),
                "+4521762615".to_string(),
            ),
            experience_list

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
        let text = vec![Self::quit_string()];

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }

    fn display_experience_widget(frame: &mut Frame, area: Rect, experience_list: &[Experience]) {
        let title = String::from("Experience");

        let mut text = vec![];

        if experience_list.is_empty() {
            text.push(Line::from(vec![Span::styled(
                "No experience added yet",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            )]));
        } else {
            for (index, experience) in experience_list.iter().enumerate() {
                // Job title med nummer
                text.push(Line::from(vec![
                    Span::styled(
                        format!("{}. ", index + 1),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        &experience.title,
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Cyan),
                    ),
                ]));

                // Period
                text.push(Line::from(vec![
                    Span::raw("    Period: "),
                    Span::styled(
                        format!("{} - {}", experience.start_date, experience.end_date),
                        Style::default().fg(Color::Green),
                    ),
                ]));

                // Short description
                text.push(Line::from(vec![
                    Span::raw("    Summary: "),
                    Span::styled(&experience.description, Style::default().fg(Color::Yellow)),
                ]));

                // Full description (truncated if too long)
                let desc = if experience.description.len() > 100 {
                    format!("{}...", &experience.description[..97])
                } else {
                    experience.description.clone()
                };
                text.push(Line::from(vec![
                    Span::raw("    Details: "),
                    Span::raw(desc),
                ]));

                // Skills hvis de findes - vis dem pænt
                if !experience.skills.is_empty() {
                    text.push(Line::from(vec![Span::raw("    Skills: ")]));

                    // Vis skills i chunks af 3-4 per linje
                    let mut skill_names = Vec::new();
                    for skill in &experience.skills {
                        // Antager at Skill har et navn felt - tilpas efter din Skill struct
                        skill_names.push(format!("• {}", skill.name)); // eller skill.title, skill.skill_name etc.
                    }

                    // Opdel skills i linjer (3-4 skills per linje)
                    for chunk in skill_names.chunks(3) {
                        let skills_line = chunk.join("  ");
                        text.push(Line::from(vec![
                            Span::raw("      "),
                            Span::styled(skills_line, Style::default().fg(Color::Magenta)),
                        ]));
                    }
                }

                // Tom linje mellem entries
                text.push(Line::from(vec![]));
            }
        }

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }
    pub fn display_widget(frame: &mut Frame, area: Rect) {
        let all_info = Self::all_info();

        // Split the main area into sections
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30), // Top section for personal/contact info
                Constraint::Percentage(50), // Middle section for experience
                Constraint::Percentage(20), // Bottom section for instructions
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
        Self::display_experience_widget(frame, main_chunks[1], &all_info.experience_list);
        Self::display_instructions_widget(frame, main_chunks[2]);
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
