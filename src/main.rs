use motoro_cli::Cli;

fn main() {
    // setup the other things here such as logging or terminal things

    if let Err(e) = Cli::run() {
        eprintln!("Motoro error: {e}");
        std::process::exit(1);
    }
}
