mod auth;
mod cli;
mod dialog;
mod resources;
mod utils;
mod view_gh;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Motoro error: {e}");
        std::process::exit(1);
    }
}
