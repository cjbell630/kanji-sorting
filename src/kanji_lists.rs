use std::collections::HashMap;

const KANJI_LISTS_JSON: &str = include_str!("../data/kanji_lists.json");

pub fn get_kanji_lists() -> HashMap<String, Vec<String>> {
    serde_json::from_str(KANJI_LISTS_JSON)
        .expect("improper JSON format in KANJI_LISTS_JSON")
}
