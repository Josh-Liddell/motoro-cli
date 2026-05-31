use crate::{
    dialog::home_dialoguer,
    resources,
    utils::{self, print_logo},
    view_gh,
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Args {
    // #[arg(short, long)]
    // example: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// View the project task board on github
    Tasks,
    /// Print out the links to learning resources (from telegram)
    Resources,
    /// Starts julia repl in motoro project
    Julia,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Tasks) => view_gh::view_tasks()?,
        Some(Commands::Resources) => resources::fetch_links()?,
        Some(Commands::Julia) => utils::start_julia_repl()?,
        None => {
            // start julia runtime on another thread? idk
            // IF we want rust to be able to execute julia stuff
            print_logo();
            home_dialoguer()?;
        }
    }

    Ok(())
}
