fn main() {
    // setup the other things here such as logging or terminal things

    if let Err(e) = motoro_cli::run() {
        eprintln!("Motoro error: {e}");
        std::process::exit(1);
    }
}

// TODO
// - fix the error handling
