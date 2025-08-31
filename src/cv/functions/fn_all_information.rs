use crate::cv::structs::cv_data::{
    AllInformation, ContactInformation, Experience, PersonalInformation, Skill,
};
use uuid::Uuid;

impl AllInformation {
    pub fn new(
        id: Uuid,
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
        let skill_list_rust = vec![Skill::new(
            Uuid::new_v4(),
            "Rust".to_string(),
            "My experience with rust".to_string(),
        )];
        let experience_list = vec![
            Experience::new(
                Uuid::new_v4(),
                "test dev title".to_string(),
                "very short description".to_string(),
                "04-03-2016".to_string(),
                "04-03-2018".to_string(),
                skill_list_rust,
            ),
            Experience::new(
                Uuid::new_v4(),
                "test dev title".to_string(),
                "very short description".to_string(),
                "04-03-2016".to_string(),
                "04-03-2018".to_string(),
                vec![Skill::new(
                    Uuid::new_v4(),
                    "C#".to_string(),
                    "test".to_string(),
                )],
            ),
            Experience::new(
                Uuid::new_v4(),
                "test dev title".to_string(),
                "very short description".to_string(),
                "04-03-2016".to_string(),
                "04-03-2018".to_string(),
                vec![Skill::new(
                    Uuid::new_v4(),
                    "C#".to_string(),
                    "test".to_string(),
                )],
            ),
            Experience::new(
                Uuid::new_v4(),
                "test dev title".to_string(),
                "very short description".to_string(),
                "04-03-2016".to_string(),
                "04-03-2018".to_string(),
                vec![Skill::new(
                    Uuid::new_v4(),
                    "C#".to_string(),
                    "test".to_string(),
                )],
            ),
        ];

        AllInformation::new(
            Uuid::new_v4(),
            PersonalInformation::new(
                Uuid::new_v4(),
                "Andreas".to_string(),
                "Barbesgaard".to_string(),
                "Software Developer".to_string(),
                36,
                "best dev".to_string(),
               "Software developer skilled in .NET (C#), React, and TypeScript. Combining technical expertise with a background in pedagogy, I create solutions that are both functional and human-centered.".to_string(),
            ),
            ContactInformation::new(
                Uuid::new_v4(),
                "Abarbesgaard@gmail.com".to_string(),
                "+4521762615".to_string(),
            ),
            experience_list

        )
    }

    fn display_personal_info() {
        let title = String::from("Personal Information");
        //Name
        //Title
        //age
        //description
        //
        println!("{}", title);
    }

    fn display_contact_information() {
        let title = String::from("Contact Information");
        //email
        //phone
        //
        println!("{}", title);
    }

    fn display_experience_information() {
        let experience_list = AllInformation::all_info().experience_list;

        for (index, experience) in experience_list.iter().enumerate() {
            // Job title med nummer
            println!("{},Job {}", index, experience.title);

            // Period

            // Short description

            // Full description (truncated if too long)

            // Skills hvis de findes - vis dem pænt
            if !experience.skills.is_empty() {

                // Vis skills i chunks af 3-4 per linje
                // Antager at Skill har et navn felt - tilpas efter din Skill struct

                // Opdel skills i linjer (3-4 skills per linje)
            }

            // Tom linje mellem entries
        }
    }
    pub fn display() {
        AllInformation::display_experience_information();
        AllInformation::display_contact_information();
        AllInformation::display_personal_info();
    }
}
