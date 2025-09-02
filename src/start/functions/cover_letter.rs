use crate::{
    cv::functions::wrap_text::wrap_text,
    start::structs::cover_letter::{Company, CoverLetter, Paragraf, Person},
};

impl CoverLetter {
    pub fn new(
        position: String,
        company: Company,
        contact_person: Person,
        paragrafs: Vec<Paragraf>,
    ) -> Self {
        CoverLetter {
            position,
            company,
            contact_person,
            paragrafs,
        }
    }

    pub fn display_coverletter(company_name: &str, is_english: bool) {
        let map = CoverLetter::info();
        if is_english == true {
            match map.get(company_name) {
                Some(buidler) => {
                    let info = buidler();
                    for paragraf in info.paragrafs {
                        let wrapped = wrap_text(&paragraf.da_text, 80);
                        println!("\t{}", wrapped);
                        println!("");
                    }
                }
                None => eprintln!("Unknown company {}", company_name),
            }
        } else {
            match map.get(company_name) {
                Some(buidler) => {
                    let info = buidler();
                    for paragraf in info.paragrafs {
                        let wrapped = wrap_text(&paragraf.en_text, 80);
                        println!("\t{}", wrapped);
                        println!("");
                    }
                }
                None => eprintln!("Unknown company {}", company_name),
            }
        }
    }
}
