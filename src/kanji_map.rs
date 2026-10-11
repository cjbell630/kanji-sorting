use std::cmp::max;
use std::collections::HashMap;

const MASTER_JSON: &str = include_str!("../data/kanji_radical_master.json");

pub(crate) type KanjiRadicalMap = HashMap<String, Vec<String>>;

type KanjiDepthMap = HashMap<String, usize>;

pub fn get_kanji_map()->KanjiRadicalMap{
    serde_json::from_str(MASTER_JSON)
        .expect("improper JSON format in MASTER_JSON")
}

pub struct KanjiRadicalMapWrapper {
    full_map: KanjiRadicalMap,
    depths: KanjiDepthMap,
}

impl KanjiRadicalMapWrapper {
    pub(crate) fn new() -> Self {
        Self {
            full_map: get_kanji_map(),
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

    fn update_lowest_depth(depths: &mut KanjiDepthMap, kanji: &String, depth: usize) {
        // TODO could count the number in each depth so memory can be statically allocated for the stringbuilders
        // TODO also track the overall lowest depth for the same reason
        match depths.get_mut(kanji) {
            Some(prev_depth) => {
                *prev_depth = max(depth, *prev_depth);
            }
            None => {
                depths.insert(kanji.clone(), depth);
            }
        }
    }

    fn parse_kanji_and_children(
        full_map: &KanjiRadicalMap,
        depths: &mut KanjiDepthMap,
        kanji: &String,
        depth: usize,
    ) {
        Self::update_lowest_depth(depths, kanji, depth);

        if let Some(radicals) = full_map.get(kanji) {
            for radical in radicals {
                Self::parse_kanji_and_children(full_map, depths, radical, depth + 1);
            }
        }
    }

    pub fn check_depths_from_targets(&mut self, targets: &[String]) {
        for target in targets {
            Self::parse_kanji_and_children(&self.full_map, &mut self.depths, target, 0);
        }
    }

    pub(crate) fn get_depth_based_order(&mut self) -> String {
        let mut strings: HashMap<usize, String> = HashMap::new();
        let mut absolute_lowest_depth = 0;

        for (kanji, depth) in &self.depths {
            match strings.get_mut(&depth) {
                Some(string) => {
                    *string += &*kanji;
                }
                None => {
                    strings.insert(*depth, kanji.parse().unwrap());
                    absolute_lowest_depth = max(absolute_lowest_depth, *depth);
                }
            }
        }

        let mut result = String::new();
        for depth in (0..=absolute_lowest_depth).rev() {
            result += strings.get(&depth).expect("skipped a depth");
        }
        result
    }

    pub(crate) fn tests_temp(&mut self) {
        self.check_depths_from_targets(&["話".parse().unwrap()]);
        println!("{:?}", self.full_map);
        println!("{:?}", self.depths);
        println!("{:?}", self.get_depth_based_order());
        self.remove_studied(&["話"]);
        println!("{:?}", self.full_map);
        Self::update_lowest_depth(&mut self.depths, &"舌".parse().unwrap(), 0);
        println!("{:?}", self.depths);
        Self::update_lowest_depth(&mut self.depths, &"舌".parse().unwrap(), 9);
        println!("{:?}", self.depths);
        Self::parse_kanji_and_children(&self.full_map, &mut self.depths, &"舌".parse().unwrap(), 0);
        println!("{:?}", self.full_map);
        println!("{:?}", self.depths);
    }
}