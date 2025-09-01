use crate::cv::structs::cv_data::ContactInformation;

impl ContactInformation {
    pub fn new(id: u32, email: String, phone_number: String) -> Self {
        Self {
            id,
            email,
            phone_number,
        }
    }

    fn c_info() -> ContactInformation {
        ContactInformation::new(
            1,
            "Abarbesgaard@gmail.com".to_string(),
            "+45 21 76 26 15".to_string(),
        )
    }

    pub fn display_widget() {
        let _info = ContactInformation::c_info();
    }
}
