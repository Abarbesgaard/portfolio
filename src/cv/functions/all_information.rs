use crate::cv::{
    functions::traits::Display,
    structs::{
        all_information::AllInformation, contact_information::ContactInformation,
        experience::Experience, personalinformation::PersonalInformation,
    },
};

impl Display for AllInformation {
    fn display() {
        PersonalInformation::display();
        Experience::display();
        ContactInformation::display();
    }

    fn display_with_details() {
        todo!();
    }
}
