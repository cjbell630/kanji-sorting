mod data_checks;

mod kanji_map;

mod kanji_lists;

use clap::Parser;

/// Creates a study order for kanji based on radical order.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Names of the lists of kanji to parse for study
    #[arg(short, long, required=true, num_args = 0..)]
    target_lists: Vec<String>,

    /// Names of the lists of kanji to skip for study
    #[arg(short, long, required=false, num_args = 0..)]
    studied_lists: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    assert!(args.target_lists.len() > 0, "Please provide at least one list of kanji to study.");

    let mut map_wrapper = kanji_map::KanjiRadicalMapWrapper::new();
    let kanji_lists = kanji_lists::get_kanji_lists();

    for target_list_name in args.target_lists {
        match kanji_lists.get(&target_list_name) {
            Some(target_list) =>  map_wrapper.check_depths_from_targets(target_list),
            None => panic!("could not find list \"{}\"",target_list_name)
        }
    }
    println!("{:?}", map_wrapper.get_depth_based_order());
}
