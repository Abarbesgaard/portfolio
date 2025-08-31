use crate::cv::structs::cv_data::{Experience, Skill};
use uuid::Uuid;

impl Experience {
    pub fn new(
        id: Uuid,
        title: String,
        description: String,
        start_date: String,
        end_date: String,
        skills: Vec<Skill>,
    ) -> Experience {
        Experience {
            id,
            title,
            description,
            start_date,
            end_date,
            skills,
        }
    }
}
