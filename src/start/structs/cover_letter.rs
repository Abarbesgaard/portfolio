#[derive(Debug)]
pub struct CoverLetter {
    pub position: String,
    pub company: Company,
    pub paragrafs: Vec<Paragraf>,
    pub contact_person: Person,
}

#[derive(Debug)]
pub struct Paragraf {
    pub da_text: String,
    pub en_text: String,
}

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub adress: String,
    pub phone_number: String,
    pub email: String,
}
