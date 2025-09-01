#[derive(Debug)]
pub struct AllInformation {
    pub id: u32,
    pub personal_information: PersonalInformation,
    pub contact_information: ContactInformation,
    pub experience_list: Vec<Experience>,
}

#[derive(Debug)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Experience {
    pub id: u32,
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub detailed_description: String,
    pub start_date: String,
    pub end_date: String,
    pub duration: String,
    pub link: Link,
    pub skills: Vec<Skill>,
}

#[derive(Debug)]
pub struct PersonalInformation {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub age: u8,
    pub tag_line: String,
    pub short_description: String,
}

#[derive(Debug)]
pub struct ContactInformation {
    pub id: u32,
    pub email: String,
}

#[derive(Debug)]
pub struct Link {
    pub id: u32,
    pub name: String,
    pub reference: String,
}

#[derive(Debug)]
pub struct Education {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub skills: Vec<Skill>,
}
