use crate::start::structs::cover_letter::Person;

impl Person {
    pub fn new(first_name: String, last_name: String, phone_number: String, email: String) -> Self {
        Person {
            first_name,
            last_name,
            phone_number,
            email,
        }
    }
}
