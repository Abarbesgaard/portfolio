use crate::cv::structs::skill::Skill;

impl Skill {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Skill {
            id,
            name,
            description,
        }
    }
}
