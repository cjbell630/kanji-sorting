// TODO write tests to check the integrity of all json data

#[cfg(test)]
mod data_checks {
    use crate::kanji_map::{KanjiRadicalMap, get_kanji_map};

    /// TODO robust but inefficient
    fn check_for_top_in_radicals(target: &String, map: &KanjiRadicalMap, mut in_stack: Vec<String>, depth:usize) {
        if(depth>100){
            panic!("depth reached on: {:?}", in_stack);
        }
        in_stack.push(target.clone());
        match map.get(target) {
            Some(radicals) => {
                for radical in radicals {
                    if(in_stack.contains(radical)){
                        panic!("cycle detected on {}. Parents: {:?}", radical, in_stack);
                    }
                    check_for_top_in_radicals(radical, map, in_stack.clone(), depth+1);
                }
            }
            None => {
                panic!(
                    "{:?} is listed as a radical but is not present as a key in master list",
                    target
                )
            }
        }
    }

    #[test]
    fn check_kanji_map() {
        let map = &get_kanji_map();
        for (kanji, radicals) in map {
            check_for_top_in_radicals(kanji, map, Vec::new(), 0)
        }
    }
}
