use crate::cv::structs::cv_data::AllInformation;
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
    Contact {
        #[arg(short, long)]
        subject: Option<String>,

        #[arg(short, long)]
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
            }
            Command::ContactInfo => {
                println!("Showing contactInformaion");
            }
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
