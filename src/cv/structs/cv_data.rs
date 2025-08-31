use uuid::Uuid;

#[derive(Debug)]
pub struct AllInformation {
    pub id: Uuid,
    pub personal_information: PersonalInformation,
    pub contact_information: ContactInformation,
}

#[derive(Debug)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
#[derive(Debug)]
pub struct Experience {
    pub id: Uuid,
    pub title: String,
    pub short_description: String,
    pub description: String,
    pub skills: Vec<Skill>,
}

#[derive(Debug)]
pub struct PersonalInformation {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub age: u8,
    pub tag_line: String,
    pub description: String,
    pub short_description: String,
}

#[derive(Debug)]
pub struct ContactInformation {
    pub id: Uuid,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug)]
pub struct Link {
    pub id: Uuid,
    pub name: String,
    pub reference: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Education {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub skills: Vec<Skill>,
}
