use crate::cv::structs::cv_data::{
    AllInformation, ContactInformation, Experience, PersonalInformation,
};

impl AllInformation {
    pub fn display() {
        PersonalInformation::display_personal_info();
        Experience::display_experience_information();
        ContactInformation::display_contact_information();
    }
}
