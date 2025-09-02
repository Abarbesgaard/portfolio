use crate::cv::functions::wrap_text;
use crate::cv::structs::cv_data::{BulletPoint, PersonalInformation};
use colored::Colorize;

impl PersonalInformation {
    pub fn new(
        id: u32,
        first_name: String,
        last_name: String,
        title: String,
        age: u8,
        tag_line: String,
        short_description: String,
        long_description: String,
        bullet_points: Vec<BulletPoint>,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            title,
            age,
            tag_line,
            short_description,
            long_description,
            bullet_points,
        }
    }
    pub fn display_personal_info() {
        let info = PersonalInformation::info();
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
        println!("\t{}", wrap_text::wrap_text(&info.short_description, 80));
        println!();
    }
    pub fn display_personal_info_with_details() {
        let info = PersonalInformation::info();

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
        println!("\t{}", wrap_text::wrap_text(&info.long_description, 80));
        println!();

        if !info.bullet_points.is_empty() {
            println!("\t{}", "What I bring:".bold());
            for bp in &info.bullet_points {
                println!(
                    "\t  {} {}",
                    format!("{}. ", bp.number).bold(),
                    bp.description
                );
            }
            println!();
        }
    }
}
