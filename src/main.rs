mod cli;
mod dialog;
mod github;
mod utils;

use cli::Cli;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("Motoro error: {e}");
        std::process::exit(1);
    }
}
