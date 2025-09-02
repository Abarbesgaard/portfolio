use crate::cv::structs::cv_data::ContactInformation;

use inquire::Text;
use opener;
use urlencoding::encode;

use colored::Colorize;

impl ContactInformation {
    pub fn new(id: u32, email: String) -> Self {
        Self { id, email }
    }
    fn info() -> ContactInformation {
        ContactInformation::new(1, "abarbesgaard@gmail.com".to_string())
    }
    pub fn display_contact_information() {
        let info = ContactInformation::info();
        println!("{}", "════════════════════".bold());
        println!("{}", "Contact Information".bold());
        println!("{}", "════════════════════\n".bold());
        println!("\t{}: {}", "Email".bold(), info.email);
        println!("\t ... or send an email directly using the 'contact' command");
        println!("\t e.g. portfolio contact");
    }

    pub fn contact(subject: Option<String>, email: Option<String>) {
        println!(
            "You are writing an email to {}",
            ContactInformation::info().email
        );
        let subject = subject.unwrap_or_else(|| Text::new("Subject:").prompt().unwrap());
        let sender = email.unwrap_or_else(|| Text::new("Your email:").prompt().unwrap());
        let body = Text::new("Message:").prompt().unwrap();
        let formatted_body = format!("From: {}\n\r\n{}", sender, body);
        let subject_enc = encode(&subject);
        let body_enc = encode(&formatted_body);
        let mailto = format!(
            "mailto:{}?subject={}&body={}",
            ContactInformation::info().email,
            subject_enc,
            body_enc
        );

        println!("Opening mail client...");
        if let Err(e) = opener::open(&mailto) {
            eprintln!("Could not open mail client: {e}");
        }
    }
}
