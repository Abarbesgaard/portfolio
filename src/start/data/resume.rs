use std::collections::HashMap;

use crate::start::structs::cover_letter::{Company, Paragraf, Person, CoverLetter};

impl CoverLetter {
    /// This is the resumé for `Vipps MobilePay`
    fn info_mobilepay() -> Self {
        CoverLetter::new(
                "Backend Engineer".to_string(),
            Company::new(
                "Vipps MobilePay".to_string(),
                "Kalkværksvej 5, 8000 Aarhus C".to_string(),
                "unkown".to_string(),
                "unkown".to_string(),
            ),
            Person::new(
                "Svend".to_string(),
                "Hyldtoft".to_string(),
                "Un".to_string(),
                "svend.hyltoft.knudsen@vippsmobilepay.com".to_string(),
            ),
            vec![Paragraf::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),

            Paragraf::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            Paragraf::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            Paragraf::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            ],
        )
    }
    fn info_company2() -> Self {
        Resume::new(
            Company::new(
                "company1".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ),
            Person::new(
                "first name".to_string(),
                "last name".to_string(),
                "phone".to_string(),
                "email.com".to_string(),
            ),
            vec![Paragraf::new("sljknsfl".to_string())],
        )
    }

    pub fn info() -> HashMap<String, fn() -> Resume> {
        let mut map: HashMap<String, fn() -> Resume> = HashMap::new();
        map.insert(
            Resume::info_mobilepay().company.name,
            Resume::info_mobilepay,
        );
        map.insert("company2".to_string(), Resume::info_company2);
        map
    }
}
