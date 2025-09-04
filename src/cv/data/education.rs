use crate::cv::structs::{education::Education, skill::Skill};

impl Education {
    pub fn new(id: u32, name: String, description: String, skills: Vec<Skill>) -> Self {
        Education {
            id,
            name,
            description,
            skills,
        }
    }

    pub fn info() -> Education {
        Education::new(
            1,
            "test".to_string(),
            "description".to_string(),
            vec![
                Skill::new(1, "stuff".to_string(), "description".to_string()),
                Skill::new(2, "name2".to_string(), "description".to_string()),
            ],
        )
    }
}
