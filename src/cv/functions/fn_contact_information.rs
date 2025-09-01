use crate::cv::structs::cv_data::ContactInformation;
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
}
