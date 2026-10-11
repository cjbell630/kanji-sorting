// TODO I want a better name for this

use crate::{kanji_lists, kanji_map};

pub fn sort(
    target_lists: Vec<String>,
    opt_studied_lists: Option<Vec<String>>,
    remove_studied_children: bool,
    _group_radicals_with_parents: bool,
    // TODO implement this
) {
    let mut map_wrapper = kanji_map::KanjiRadicalMapWrapper::new();
    let kanji_lists = kanji_lists::get_kanji_lists();

    if let Some(studied_lists) = opt_studied_lists {
        for studied_list_name in studied_lists {
            match kanji_lists.get(&studied_list_name) {
                Some(studied_list) => {
                    // TODO research terns
                    if remove_studied_children {
                        map_wrapper.remove_and_children_from_map(studied_list);
                    } else {
                        map_wrapper.remove_from_map(studied_list);
                    }
                }
                None => panic!("unknown list \"{}\"", studied_list_name),
            }
        }
    }

    // TODO very similar to above
    for target_list_name in target_lists {
        match kanji_lists.get(&target_list_name) {
            Some(target_list) => map_wrapper.check_depths_from_targets(target_list),
            None => panic!("unknown list \"{}\"", target_list_name),
        }
    }

    println!("{:?}", map_wrapper.get_depth_based_order());
}
