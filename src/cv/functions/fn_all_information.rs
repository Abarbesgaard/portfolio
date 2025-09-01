use crate::cv::structs::cv_data::{
    AllInformation, ContactInformation, Experience, PersonalInformation, Skill,
};
use colored::Colorize;

impl AllInformation {
    pub fn new(
        id: u32,
        personal_information: PersonalInformation,
        contact_information: ContactInformation,
        experience_list: Vec<Experience>,
    ) -> Self {
        Self {
            id,
            personal_information,
            contact_information,
            experience_list,
        }
    }

    fn all_info() -> AllInformation {
        let experience_list = vec![
            Experience::new(
                1,
                "Mentor".to_string(),
                "I am a mentor for six individuals in computer science (datamatiker) programs across Denmark.".to_string(),
                "01-03-2025".to_string(),
                "01-06-2025".to_string(),
                "4 months".to_string(),
                vec![Skill::new(
                    11,
                    "Relationship building".to_string()), 
                    Skill::new(
                        12,
                        "Coaching".to_string()),
                    Skill::new(
                        13,
                        "Teaching".to_string())],
            ),
            Experience::new(
                2,
                "Intern".to_string(),
                "very short description".to_string(),
                "04-03-2016".to_string(),
                "04-03-2018".to_string(),
                "3 months".to_string(),
                vec![Skill::new(
                    21,
                    "Entity Framework".to_string()), 
                    Skill::new(
                        23,
                        "jQuery".to_string())]
            ),

                   ];

        AllInformation::new(
            1,
            PersonalInformation::new(
                11,
                "Andreas".to_string(),
                "Barbesgaard".to_string(),
                "Software Developer".to_string(),
                36,
                "best dev".to_string(),
                "Software developer skilled in .NET (C#), React, and TypeScript. Combining technical expertise with a background in pedagogy, I create solutions that are both functional and human-centered.".to_string(),
            ),
            ContactInformation::new(
                1,
                "Abarbesgaard@gmail.com".to_string(),
                "+4521762615".to_string(),
            ),
            experience_list

        )
    }

    fn display_personal_info() {
        let info = AllInformation::all_info().personal_information;
        println!("{}", "════════════════════".bold());
        println!("{}", "Personal Information".bold());
        println!("{}", "════════════════════\n".bold());

        println!(
            "\t{}: {} {}",
            "Name".bold(),
            info.first_name,
            info.last_name
        );

        println!("\t{}: {}", "Title".bold(), info.title);
        println!("\t{}: {}", "Age".bold(), info.age);
        println!(
            "\t{}",
            AllInformation::wrap_text(&info.short_description, 80)
        );
        println!();
    }

    fn display_contact_information() {
        let info = AllInformation::all_info().contact_information;
        println!("{}", "════════════════════".bold());
        println!("{}", "Contact Information".bold());
        println!("{}", "════════════════════\n".bold());
        println!("\t{}: {}", "Email".bold(), info.email);
        println!("\t ... or send an email directly using the 'contact' command");
        println!("\t e.g. portfolio contact");
    }
    fn wrap_text(text: &str, width: usize) -> String {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > width {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines.join("\n\t")
    }
    fn display_experience_information() {
        let experience_list = AllInformation::all_info().experience_list;
        println!("{}", "═══════════".bold());
        println!("{}", "Experience".bold());
        println!("{}", "═══════════\n".bold());

        for (_i, experience) in experience_list.iter().enumerate() {
            println!("\t{}", experience.title.bold());
            println!(
                "\t{} - {}: {} \n",
                experience.start_date, experience.end_date, experience.duration
            );
            println!("\t{}", "Description".bold());
            if experience.description.len() >= 80 {
                println!(
                    "\t{}",
                    AllInformation::wrap_text(&experience.description, 80)
                );
            } else {
                println!("\t{}", experience.description);
            }
            if !experience.skills.is_empty() {
                println!("\t{}", "Aquired Skills".italic());
                // Vis skills i chunks af 3-4 per linje
                for (index, skill) in experience.skills.iter().enumerate() {
                    if index == 0 {
                        print!("{}", "\t\t╔".blue());
                        println!(" {}", skill.name);
                    } else if index != 0 && index != experience.skills.len() - 1 {
                        print!("{}", "\t\t╟".blue());
                        println!(" {}", skill.name);
                    } else {
                        print!("{}", "\t\t╚".blue());
                        println!(" {}", skill.name);
                    }
                }
            }
            println!();
        }
    }
    pub fn display() {
        AllInformation::display_personal_info();
        AllInformation::display_experience_information();
        AllInformation::display_contact_information();
    }
}
