use colored::Colorize;

use crate::cv::functions::helper::wrap_text::wrap_text;
use crate::cv::functions::traits::Display;
use crate::cv::structs::education::Education;

impl Display for Education {
    fn display() {
        let info = Education::info();

        for (index, education) in info.iter().enumerate() {
            print!("{} ", index);
            println!("{}", education.name.bold());
            println!("\t{}", wrap_text(&education.description, 80));
            for (index, skill) in education.skills.iter().enumerate() {
                print!("\t\t{}: ", index);
                println!("{}", skill.name);
            }
            println!("");
        }
    }

    fn display_with_details() {
        let info = Education::info();
        for (index, education) in info.iter().enumerate() {
            print!("{} ", index);
            println!("{}", education.name.bold());
            println!("\t{}", wrap_text(&education.description, 81));
            for (index, skill) in education.skills.iter().enumerate() {
                print!("\t\t{}: ", index);
                println!("{}", skill.name);
            }
            println!("");
        }
    }
}
