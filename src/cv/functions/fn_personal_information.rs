use crate::cv::functions::wrap_text;
use crate::cv::structs::cv_data::PersonalInformation;
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
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            title,
            age,
            tag_line,
            short_description,
        }
    }
    fn info() -> PersonalInformation {
        PersonalInformation::new(
            1,
            "Andreas".to_string(),
            "Barbesgaard".to_string(),
            "Software Developer".to_string(),
            36,
            "C# dev with rusty tendencies".to_string(),
            " ".to_string(),
        )
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
}
