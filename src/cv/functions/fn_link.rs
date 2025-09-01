use crate::cv::structs::cv_data::Link;
impl Link {
    pub fn new(id: u32, name: String, reference: String) -> Link {
        Link {
            id,
            name,
            reference,
        }
    }
}
