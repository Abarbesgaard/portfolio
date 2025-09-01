use crate::cv::structs::cv_data::{
    AllInformation, ContactInformation, Experience, PersonalInformation,
};
use clap::{Parser, Subcommand};
use inquire::Text;
use opener;
use urlencoding::encode;

#[derive(Parser, Debug)]
#[command(name = "Portfolio")]
#[command(about = "A quick glance at my portfolio and CV")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(long_about = "This command shows all information")]
    All,
    #[command(long_about = "This command shows personal information")]
    PersonalInfo,
    #[command(long_about = "This command shows contact information")]
    ContactInfo,
    #[command(long_about = "This command shows contact information")]
    ExperienceInfo {
        #[arg(
            short,
            long,
            help = "Write the number of the experience you want to see"
        )]
        number: Option<u32>,
        #[arg(
            short,
            long,
            help = "Write the number of the experience you want to see"
        )]
        details: bool,
    },
    #[command(long_about = "This command shows contact information")]
    Contact {
        #[arg(short, long, help = "The subject of the email")]
        subject: Option<String>,

        #[arg(short, long, help = "Your email")]
        email: Option<String>,
    },
}

impl Args {
    pub fn execute() {
        let args = Args::parse();

        match args.command {
            Command::All => {
                println!("Showing all information");
                AllInformation::display();
            }
            Command::PersonalInfo => {
                println!("Showing personal information");
                PersonalInformation::display_personal_info();
            }
            Command::ContactInfo => {
                println!("Showing contactInformaion");
                ContactInformation::display_contact_information();
            }
            Command::ExperienceInfo { number, details } => match (number, details) {
                (Some(n), false) => {
                    Experience::display_experience_by_number(n);
                }
                (Some(n), true) => {
                    Experience::display_detailed_experience(n);
                }
                (None, _) => {
                    Experience::display_experience_information();
                }
            },
            Command::Contact { subject, email } => {
                println!("Contact me");
                let subject = subject.unwrap_or_else(|| Text::new("Subject:").prompt().unwrap());
                let sender = email.unwrap_or_else(|| Text::new("Your email:").prompt().unwrap());
                let body = Text::new("Message:").prompt().unwrap();
                let formatted_body = format!("From: {}\n\r\n{}", sender, body);
                let subject_enc = encode(&subject);
                let body_enc = encode(&formatted_body);
                let mailto = format!(
                    "mailto:me@example.com?subject={}&body={}",
                    subject_enc, body_enc
                );

                println!("Opening mail client...");
                if let Err(e) = opener::open(&mailto) {
                    eprintln!("Could not open mail client: {e}");
                }
            }
        }
    }
}
