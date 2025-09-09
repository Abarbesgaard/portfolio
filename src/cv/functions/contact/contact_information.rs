use crate::cv::{functions::traits::Display, structs::contact_information::ContactInformation};

impl Display for ContactInformation {
    fn display() {
        Self::banner();
        Self::title();
        Self::info();
        Self::footer();
    }

    fn display_with_details() {
        Self::banner();
        Self::title();
        Self::info();
        Self::footer();
    }
}
impl ContactInformation {
    pub fn info() -> Self {
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
}
