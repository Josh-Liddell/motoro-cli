use crate::{resources, utils, view_gh};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Args {
    // #[arg(short, long)]
    // example: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the logo for fun
    Logo,
    /// View the project task board on github
    Tasks,
    /// Print out the links to learning resources (from telegram)
    Resources,
    // Test,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Logo => utils::print_logo(),
        Commands::Tasks => view_gh::view_tasks()?,
        Commands::Resources => resources::fetch_links()?,
        // Commands::Test => view_gh::view_projects()?,
    }

    Ok(())
}
