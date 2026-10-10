use std::collections::HashMap;

const MASTER_JSON: &str = include_str!("../data/kanji_radical_master.json");

type KanjiRadicalMap = HashMap<String, Vec<String>>;

type KanjiDepthMap = HashMap<String, i32>;

struct KanjiRadicalMapWrapper {
    full_map: KanjiRadicalMap,
    depths: KanjiDepthMap,
}

impl KanjiRadicalMapWrapper {
    fn new() -> Self {
        Self {
            full_map: serde_json::from_str(MASTER_JSON)
                .expect("improper JSON format in MASTER_JSON"),
            // TODO make separate function so it can be used from the command line,
            // change error message to match, move MASTER_JSON to some sort of dict with path/data
            // associations for error processing
            depths: HashMap::new(),
        }
    }

    fn remove_studied(&mut self, studied: &[impl AsRef<str>]) {
        for kanji in studied {
            self.full_map.remove(kanji.as_ref());
        }
    }

    fn update_lowest_depth(&mut self, kanji: String, depth: i32) {
        if self
            .depths
            .get(&kanji)
            .is_some_and(|prev_depth| &depth <= prev_depth)
        {
            return;
        }
        self.depths.insert(kanji, depth);
    }

    fn h(&mut self) {}
}

fn main() {
    let mut bruh = KanjiRadicalMapWrapper::new();
    println!("{:?}", bruh.full_map);
    bruh.remove_studied(&["話"]);
    println!("{:?}", bruh.full_map);
    bruh.update_lowest_depth("舌".parse().unwrap(), 0);
    println!("{:?}", bruh.depths);
    bruh.update_lowest_depth("舌".parse().unwrap(), 9);
    println!("{:?}", bruh.depths);
}
