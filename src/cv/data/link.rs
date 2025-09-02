use crate::cv::structs::cv_data::Link;

impl Link {
    pub fn new(
        id: u32,
        name: String,
        reference: String,
        short_description: String,
        long_description: String,
    ) -> Link {
        Link {
            id,
            name,
            reference,
            short_description,
            long_description,
        }
    }

    pub fn info() -> Vec<Link> {
        let links = vec![Link::new(
            1,
            "name".to_string(),
            "reference".to_string(),
            "short description".to_string(),
            "long description".to_string(),
        )];
        links
    }
}
