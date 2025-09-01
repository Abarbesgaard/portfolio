use crate::cv::structs::cv_data::{Experience, Skill};

impl Experience {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        start_date: String,
        end_date: String,
        duration: String,
        skills: Vec<Skill>,
    ) -> Experience {
        Experience {
            id,
            title,
            description,
            start_date,
            end_date,
            duration,
            skills,
        }
    }
}
