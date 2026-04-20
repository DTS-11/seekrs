mod cli;
mod display;
mod duplicate;
mod filters;
mod opener;
mod preview;
mod search;
mod tree;
mod upgrader;

use clap::Parser;
use cli::Args;

fn main() {
    // Parse all CLI flags typed by the user into our Args struct
    let args = Args::parse();

    display::print_banner();

    if args.upgrade {
        upgrader::upgrade();
    } else if args.duplicates {
        duplicate::find_duplicates(&args);
    } else if args.tree {
        tree::print_tree(&args);
    } else {
        search::run_search(&args);
    }
}
