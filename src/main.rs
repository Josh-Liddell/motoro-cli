mod auth;
mod cli;
mod dialog;
mod resources;
mod utils;
mod view_gh;

use std::env;

fn main() {
    let result = if env::args().len() == 1 {
        utils::print_logo();
        dialog::home_dialoguer()
    } else {
        cli::run()
    };

    if let Err(e) = result {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
