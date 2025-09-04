use crate::cv::structs::{
    contact_information::ContactInformation, experience::Experience,
    personalinformation::PersonalInformation,
};
#[derive(Debug)]
pub struct AllInformation {
    pub id: u32,
    pub personal_information: PersonalInformation,
    pub contact_information: ContactInformation,
    pub experience_list: Vec<Experience>,
}
