use clap::{Parser, Subcommand};

use crate::cv::structs::cv_data::AllInformation;

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
        }
    }
}
