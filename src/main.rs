mod cli;
mod display;
mod filters;
mod search;
mod preview;
mod duplicate;
mod tree;
mod opener;

use clap::Parser;
use cli::Args;

fn main() {
    // Parse all CLI flags typed by the user into our Args struct
    let args = Args::parse();

    // Print the big colorful banner
    display::print_banner();

    // Route to the right feature
    if args.duplicates {
        duplicate::find_duplicates(&args);
    } else if args.tree {
        tree::print_tree(&args);
    } else {
        search::run_search(&args);
    }
}
