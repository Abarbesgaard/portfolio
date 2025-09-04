use crate::cv::functions::helper::wrap_text;
use crate::cv::functions::traits::Display;
use crate::cv::structs::experience::Experience;
use colored::Colorize;

impl Display for Experience {
    fn display() {
        let experience_list = Experience::info();
        println!("{}", "═══════════".bold());
        println!("{}", "Experience".bold());
        println!("{}", "═══════════\n".bold());

        for (_i, experience) in experience_list.iter().enumerate() {
            println!("\t{} -> {}", "Id".bold(), experience.id);
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
            println!("\t┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈");
        }
    }
    fn display_with_details() {
        todo!("not implemented");
    }
}
impl Experience {
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
            }
        }
    }
}
