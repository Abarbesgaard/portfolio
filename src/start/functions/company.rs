use crate::start::structs::cover_letter::Company;

impl Company {
    pub fn new(name: String, adress: String, phone_number: String, email: String) -> Self {
        Company {
            name,
            adress,
            phone_number,
            email,
        }
    }
}
