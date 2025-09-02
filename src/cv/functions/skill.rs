use crate::cv::structs::cv_data::Skill;

impl Skill {
    pub fn new(id: u32, name: String, description: String) -> Skill {
        Skill {
            id,
            name,
            description,
        }
    }
}
