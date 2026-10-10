use std::collections::HashMap;

const MASTER_JSON: &str = include_str!("../data/kanji_radical_master.json");

type KanjiRadicalMap = HashMap<String, Vec<String>>;


struct KanjiRadicalMapWrapper {
    map: KanjiRadicalMap,
}

impl KanjiRadicalMapWrapper {
    fn new() -> Self {
        Self {
            map: serde_json::from_str(MASTER_JSON).expect("improper JSON format in MASTER_JSON"),
            // TODO make separate function so it can be used from the command line, change error message to match, move MASTER_JSON to some sort of dict with path/data associations for error processing
        }
    }

    fn remove_studied(&mut self, studied: &[impl AsRef<str>]) {
        for kanji in studied {
            self.map.remove(kanji.as_ref());
        }
    }
}

fn main() {
    let mut bruh = KanjiRadicalMapWrapper::new();
    println!("{:?}", bruh.map.keys());
    bruh.remove_studied(&["話"]);
    println!("{:?}", bruh.map.keys());
}
