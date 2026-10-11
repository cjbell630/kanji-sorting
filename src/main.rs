use std::collections::HashMap;

mod kanji_map;

const KANJI_LISTS_JSON: &str = include_str!("../data/kanji_lists.json");

fn main() {
    let kanji_lists: HashMap<String, Vec<String>> = serde_json::from_str(KANJI_LISTS_JSON).expect("improper JSON format in KANJI_LISTS_JSON");
    let mut bruh = kanji_map::KanjiRadicalMapWrapper::new();
    if let Some(targets) = kanji_lists.get("N1"){
        bruh.check_depths_from_targets(targets);
        println!("{:?}", bruh.get_depth_based_order());
    }


}
