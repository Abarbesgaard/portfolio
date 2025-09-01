use crate::cv::structs::cv_data::PersonalInformation;

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

    fn p_info() -> PersonalInformation {
        PersonalInformation::new(
            1,
            "Andreas".to_string(),
            "Barbesgaard".to_string(),
            "Software Developer".to_string(),
            36,
            "best dev ever".to_string(),
            "test short description".to_string(),
        )
    }

    pub fn display_widget() {
        let _info = PersonalInformation::p_info();
    }
}
