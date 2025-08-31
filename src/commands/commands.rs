use crate::ui::view;
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
                if let Err(e) = view::show_all_view() {
                    eprintln!("Error displaying all information, {}", e);
                }
            }
            Command::PersonalInfo => {
                println!("Showing personal information");
                if let Err(e) = view::personal_information_view() {
                    eprintln!("Error displaying portfolio: {}", e);
                }
            }
            Command::ContactInfo => {
                println!("Showing contactInformaion");
                if let Err(e) = view::contact_information_view() {
                    eprintln!("Error displaying contact information: {}", e);
                }
            }
        }
    }
}
