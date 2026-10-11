mod data_checks;

mod kanji_map;

mod kanji_lists;

use clap::Parser;

/// Creates a study order for kanji based on radical order.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Names of the lists of kanji to parse for study (see --print-list-names)
    #[arg(short, long, required_unless_present_any=["print_list_names"], num_args = 0..)]
    target_lists: Vec<String>,

    /// Names of the lists of kanji to skip for study (see --print-list-names)
    #[arg(short, long, required_if_eq_any=[("remove_studied_children","true")], num_args = 0..)]
    studied_lists: Option<Vec<String>>,

    /// Print the available preset list names and exit
    #[arg(short, long, action, required = false)]
    print_list_names: bool,

    /// Print the available preset list names and exit
    #[arg(short, long, action, required = false)]
    remove_studied_children: bool,
}

fn main() {
    let args = Args::parse();

    if (args.print_list_names) {
        // TODO feels weird putting get call here (so 2 times) but objectively not redundant
        println!("Preset kanji lists:");
        println!("{:?}", kanji_lists::get_kanji_lists().keys());
        return;
    }

    assert!(
        args.target_lists.len() > 0,
        "Please provide at least one list of kanji to study."
    );

    let mut map_wrapper = kanji_map::KanjiRadicalMapWrapper::new();
    let kanji_lists = kanji_lists::get_kanji_lists();

    if let Some(studied_lists) = args.studied_lists {
        for studied_list_name in studied_lists {
            match kanji_lists.get(&studied_list_name) {
                Some(studied_list) => {
                    // TODO research terns
                    if args.remove_studied_children {
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
    for target_list_name in args.target_lists {
        match kanji_lists.get(&target_list_name) {
            Some(target_list) => map_wrapper.check_depths_from_targets(target_list),
            None => panic!("unknown list \"{}\"", target_list_name),
        }
    }

    println!("{:?}", map_wrapper.get_depth_based_order());
}
