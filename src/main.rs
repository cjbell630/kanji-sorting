use std::collections::HashMap;

mod kanji_map;

mod kanji_lists;

fn main() {
    let mut bruh = kanji_map::KanjiRadicalMapWrapper::new();
    let kanji_lists = kanji_lists::get_kanji_lists();
    if let Some(targets) = kanji_lists.get("N1") {
        bruh.check_depths_from_targets(targets);
        println!("{:?}", bruh.get_depth_based_order());
    }
}
