use crate::start::structs::cover_letter::Paragraf;

impl Paragraf {
    pub fn new(da_text: String, en_text: String) -> Self {
        Paragraf { da_text, en_text }
    }
}
