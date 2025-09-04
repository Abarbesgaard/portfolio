use crate::cv::structs::skill::Skill;

#[derive(Debug)]
pub struct Education {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub skills: Vec<Skill>,
}
