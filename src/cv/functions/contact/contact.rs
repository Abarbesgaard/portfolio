use crate::cv::structs::contact_information::ContactInformation;

use opener;
impl ContactInformation {
    pub fn contact(subject: Option<String>, email: Option<String>) {
        let info = ContactInformation::info();
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
