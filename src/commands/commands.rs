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
    #[command(long_about = "This is a very long test about what new does")]
    Show,
    ContactInfo,
}

impl Args {
    pub fn execute() {
        let args = Args::parse();

        match args.command {
            Command::Show => {
                println!("Showing portfolio");
                if let Err(e) = view::personal_information_view() {
                    eprintln!("Error displaying portfolio: {}", e);
                }
            }
            Command::ContactInfo => {
                println!("Showing contactInformaion")
            }
        }
    }
}
