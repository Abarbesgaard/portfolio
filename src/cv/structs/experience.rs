use crate::cv::structs::{link::Link, skill::Skill};

#[derive(Debug)]
pub struct Experience {
    pub id: u32,
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub detailed_description: String,
    pub start_date: String,
    pub end_date: String,
    pub duration: String,
    pub link: Link,
    pub skills: Vec<Skill>,
}
