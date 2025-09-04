use colored::Colorize;

use crate::cv::{functions::helper::wrap_text, structs::link::Link};

impl Link {
    fn banner() {
        println!("My Portfolio");
    }
    pub fn display() {
        let links = Link::info();
        Link::banner();
        let indentation = "\t".to_string();
        for link in links {
            println!("{}{}", indentation, link.name.bold());
            println!("{}{}", indentation, link.reference);
            println!(
                "{}{}",
                indentation,
                wrap_text::wrap_text(&link.short_description, 80)
            );
            println!();
        }
    }

    pub fn display_with_details() {
        let links = Link::info();
        Link::banner();
        let indentation = "\t".to_string();
        for link in links {
            println!("{}{}", indentation, link.name.bold());
            println!("{}{}", indentation, link.reference);
            println!(
                "{}{}",
                indentation,
                wrap_text::wrap_text(&link.long_description, 80)
            );
            println!();
        }
    }
}
