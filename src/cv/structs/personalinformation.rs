use crate::cv::structs::bullet_point::BulletPoint;

#[derive(Debug)]
pub struct PersonalInformation {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub age: u8,
    pub tag_line: String,
    pub short_description: String,
    pub long_description: String,
    pub bullet_points: Vec<BulletPoint>,
}
