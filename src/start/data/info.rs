use std::collections::HashMap;

use crate::start::structs::cover_letter::CoverLetter;

impl CoverLetter {
    /// This is the resumé for `Vipps MobilePay`

    pub fn info() -> HashMap<String, fn() -> CoverLetter> {
        let mut map: HashMap<String, fn() -> CoverLetter> = HashMap::new();
        map.insert(
            CoverLetter::info_mobilepay().company.name,
            CoverLetter::info_mobilepay,
        );
        map.insert(
            CoverLetter::info_databricks().company.name,
            CoverLetter::info_databricks,
        );
        map.insert(
            CoverLetter::info_uber().company.name,
            CoverLetter::info_uber,
        );
        map
    }
}
