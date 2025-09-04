use crate::cv::{functions::traits::Display, structs::contact_information::ContactInformation};

use opener;
impl Display for ContactInformation {
    fn display() {
        Self::banner();
        Self::title();
        Self::info();
        Self::footer();
    }

    fn display_with_details() {
        todo!("not implemented");
    }
}
impl ContactInformation {
    fn info() -> Self {
        ContactInformation::new(1, "abarbesgaard@gmail.com".to_string())
    }

    fn banner() {
        ContactInformation::title();
    }

    fn title() -> &'static str {
        "Contact Information"
    }

    fn footer() {
        println!("\t ... or send an email directly using the 'contact' command");
        println!("\t e.g. portfolio contact");
    }

    pub fn contact(subject: Option<String>, email: Option<String>) {
        let info = Self::info();
        println!("You are writing an email to {}", info.email);

        let subject = subject.unwrap_or_else(|| inquire::Text::new("Subject:").prompt().unwrap());
        let sender = email.unwrap_or_else(|| inquire::Text::new("Your email:").prompt().unwrap());
        let body = inquire::Text::new("Message:").prompt().unwrap();

        let formatted_body = format!("From: {}\n\r\n{}", sender, body);
        let subject_enc = urlencoding::encode(&subject);
        let body_enc = urlencoding::encode(&formatted_body);

        let mailto = format!(
            "mailto:{}?subject={}&body={}",
            info.email, subject_enc, body_enc
        );

        println!("Opening mail client...");
        if let Err(e) = opener::open(&mailto) {
            eprintln!("Could not open mail client: {e}");
        }
    }
}
