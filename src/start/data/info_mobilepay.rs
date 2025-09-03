use crate::start::structs::cover_letter::{CoverLetter, Company, Paragraf, Person} ;

impl  CoverLetter{
    pub fn info_mobilepay() -> Self {
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
            vec![Paragraf::new("".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),

            Paragraf::new("".to_string(),"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            Paragraf::new("".to_string(),"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            Paragraf::new("".to_string(),"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus volutpat iaculis ultricies. Nullam faucibus sem a magna ornare, et ullamcorper massa sodales. Nulla nunc erat, vulputate eu sapien eu, pretium vehicula justo. Nullam vestibulum interdum viverra. Nulla id leo auctor, vestibulum augue ut, consectetur enim. Donec lobortis libero non sapien auctor, at pellentesque tellus elementum. Etiam vehicula magna vel enim semper, sit amet tincidunt quam tempus.".to_string()),
            
            ],
        )
    }

}
