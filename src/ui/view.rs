use crate::cv::structs::cv_data::{AllInformation, ContactInformation, PersonalInformation};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Error};

pub fn personal_information_view() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    loop {
        terminal.draw(|f| {
            let area = f.area();
            PersonalInformation::display_widget(f, area);
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
pub fn contact_information_view() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            ContactInformation::display_widget(f, area);
        })?;
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

pub fn show_all_view() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            AllInformation::display_widget(f, area);
        })?;
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
