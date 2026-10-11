mod data_checks;

mod kanji_map;

mod kanji_lists;
mod logic;

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

    /// Recursively remove all children of kanji in the studied lists as well as the kanji themselves
    #[arg(short, long, action, required = false)]
    remove_studied_children: bool,

    /// Group childless non-joyo kanji with their first joyo parent [TODO not implemented yet]
    #[arg(short, long, action, required = false)]
    group_radicals_with_parents: bool,
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

    logic::sort(
        args.target_lists,
        args.studied_lists,
        args.remove_studied_children,
        args.group_radicals_with_parents,
    );
}
