use crate::cv::structs::all_information::AllInformation;
use crate::cv::structs::contact_information::ContactInformation;
use crate::cv::structs::education::Education;
use crate::cv::structs::experience::Experience;
use crate::cv::structs::personalinformation::PersonalInformation;
use crate::cv::{functions::traits::Display, structs::link::Link};
use crate::start::functions::start::start;
use crate::start::structs::cover_letter::CoverLetter;
use clap::{Parser, Subcommand};

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
    CoverLetter {
        #[arg(short, long, help = "Write your company name and start the program")]
        company_name: Option<String>,

        #[arg(short, long, help = "Write your company name and start the program")]
        in_english: bool,
    },

    #[command(long_about = "This command shows all information")]
    Portfolio {
        #[arg(short, long, help = "view detailed description")]
        details: bool,
    },
    #[command(long_about = "This command shows all information")]
    Start,
    #[command(long_about = "This command shows all information")]
    All,
    #[command(long_about = "This command shows personal information")]
    Education {
        #[arg(short, long, help = "view detailed description")]
        details: bool,
    },

    #[command(long_about = "This command shows personal information")]
    PersonalInfo {
        #[arg(short, long, help = "view detailed description")]
        details: bool,
    },
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
            Command::Education { details } => match details {
                true => {
                    Education::display_with_details();
                }
                false => {
                    Education::display();
                }
            },
            Command::Portfolio { details } => match details {
                true => {
                    Link::display_with_details();
                }
                false => {
                    Link::display();
                }
            },
            Command::Start => {
                start();
            }
            Command::CoverLetter {
                company_name,
                in_english,
            } => {
                if let Some(company) = company_name {
                    CoverLetter::display_coverletter(&company, in_english);
                } else {
                    println!("Write a proper company name");
                }
            }
            Command::All => {
                println!("Showing all information");
                AllInformation::display();
            }
            Command::PersonalInfo { details } => match details {
                true => {
                    PersonalInformation::display_with_details();
                }
                false => {
                    PersonalInformation::display();
                }
            },
            Command::ContactInfo => {
                println!("Showing contactInformaion");
                ContactInformation::display();
            }
            Command::ExperienceInfo { number, details } => match (number, details) {
                (Some(n), false) => {
                    Experience::display_experience_by_number(n);
                }
                (Some(n), true) => {
                    Experience::display_detailed_experience(n);
                }
                (None, _) => {
                    Experience::display();
                }
            },
            Command::Contact { subject, email } => {
                ContactInformation::contact(subject, email);
            }
        }
    }
}
