use crate::cv::functions::wrap_text;
use crate::cv::structs::cv_data::{Experience, Link, Skill};
use colored::Colorize;

impl Experience {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        start_date: String,
        end_date: String,
        detailed_description: String,
        duration: String,
        skills: Vec<Skill>,
        link: Link,
        company_name: String,
    ) -> Experience {
        Experience {
            id,
            title,
            description,
            start_date,
            end_date,
            detailed_description,
            duration,
            skills,
            link,
            company_name,
        }
    }

    fn info() -> Vec<Experience> {
        vec![
    Experience::new(
        1,
        "Mentor".to_string(),
        "I am a mentor for six individuals in computer science (datamatiker) programs across Denmark.".to_string(),
        "01-03-2025".to_string(),
        "01-06-2025".to_string(),
        "As a mentor, I supported students in their professional and academic development through regular one-on-one sessions and ongoing guidance. 
        My focus was on helping them strengthen technical skills, improve problem-solving strategies, and build confidence in real-world software development practices. 
        In addition to academic support, I emphasized personal growth by encouraging goal setting, reflective learning, and career-oriented discussions. 
        This role required adaptability, patience, and strong communication skills to meet the unique needs of each mentee, ensuring both their professional and personal progress.".to_string(),
        "4 months".to_string(),
        vec![
            Skill::new(
                11,
                "Relationship building".to_string(),
                "Building trust and strong connections".to_string(),
            ),
            Skill::new(
                12,
                "Coaching".to_string(),
                "Guiding others toward growth".to_string(),
            ),
            Skill::new(
                13,
                "Teaching".to_string(),
                "Explaining concepts in a clear way".to_string(),
            ),
        ],
        Link::new(
            1,
            "Duos".to_string(),
            "https://duos.dk/".to_string(),
        ),
        "DUOS".to_string(),
    ),
]
    }
    pub fn display_experience_information() {
        let experience_list = Experience::info();
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
                println!("\t{}", wrap_text::wrap_text(&experience.description, 80));
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
            } else {
                println!("\t\t _> wip")
            }
            println!();
        }
    }

    pub fn display_experience_by_number(number: u32) {
        let experience_list = Experience::info();

        for experience in experience_list.iter() {
            if number == experience.id {
                println!("\t{} -> {}", "Id".bold(), experience.id);
                println!("\t{}", experience.title.bold());
                println!("\t{}", experience.company_name);
                println!(
                    "\t{} - {}: {} \n",
                    experience.start_date, experience.end_date, experience.duration
                );
                println!("\t{}", "Description".bold());
                if experience.description.len() >= 80 {
                    println!("\t{}", wrap_text::wrap_text(&experience.description, 80));
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
                } else {
                    println!("\t\t _> wip")
                }
                println!();
            } else {
                println!("sorry no exp found");
            }
        }
    }
    pub fn display_detailed_experience(number: u32) {
        let experience_list = Experience::info();
        for experience in experience_list.iter() {
            if number == experience.id {
                println!("\t{} -> {}", "Id".bold(), experience.id);
                println!("\t{}", experience.title.bold());
                println!("\t{}", experience.company_name);
                println!(
                    "\t{} - {}: {} \n",
                    experience.start_date, experience.end_date, experience.duration
                );
                println!("\t{}", "Description".bold());
                if experience.description.len() >= 80 {
                    println!(
                        "\t{}",
                        wrap_text::wrap_text(&experience.detailed_description, 80)
                    );
                } else {
                    println!("\t{}", experience.description);
                }
                if !experience.skills.is_empty() {
                    println!("\t{}", "Aquired Skills".italic());
                    for (index, skill) in experience.skills.iter().enumerate() {
                        if index == 0 {
                            print!("{}", "\t\t╔".blue());
                            println!(" {}", skill.name);
                            print!("{}", "\t\t║".blue());
                            println!("\t{}", skill.description);
                        } else if index != 0 && index != experience.skills.len() - 1 {
                            print!("{}", "\t\t╟".blue());
                            println!(" {}", skill.name);
                            print!("{}", "\t\t║".blue());
                            println!("\t{}", skill.description);
                        } else {
                            print!("{}", "\t\t╟".blue());
                            println!(" {}", skill.name);
                            print!("{}", "\t\t║".blue());
                            println!("\t{}", skill.description);
                            print!("{}", "\t\t╚".blue());
                        }
                    }
                } else {
                    println!("\t\t _> wip")
                }
                println!();
            } else {
                println!("sorry no exp found");
            }
        }
    }
}
