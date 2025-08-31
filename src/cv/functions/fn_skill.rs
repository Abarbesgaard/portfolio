use crate::cv::structs::cv_data::Skill;
use uuid::Uuid;

impl Skill {
    pub fn new(id: Uuid, name: String, description: String) -> Skill {
        Skill {
            id,
            name,
            description,
        }
    }
}
