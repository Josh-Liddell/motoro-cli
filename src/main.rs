mod cli;
mod dialog;
mod utils;

use std::env;

fn main() {
    if env::args().len() == 1 {
        utils::print_logo();
        dialog::example();
    } else {
        cli::run();
    }
}
