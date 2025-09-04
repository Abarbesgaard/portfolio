use crate::cv::structs::contact_information::ContactInformation;

impl ContactInformation {
    pub fn new(id: u32, email: String) -> Self {
        Self { id, email }
    }
}
