use crate::{tasks::view, utils};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Args {
    // #[arg(short, long)]
    // status: bool,
    // #[arg(long)]
    // one: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the logo for fun
    Logo,
    /// View the project task board on github
    Tasks,
    /// Print out the links to learning resources
    Resources,
    // Adds files to myapp, ignore
    // Add { name: Option<String> },
    // ignore
    // Run,
}

pub fn run() {
    let args = Args::parse();

    match &args.command {
        // Commands::Add { name } => {
        //     println!("'add' was used, name is: {name:?}");
        // }
        // Commands::Run => println!("running"),
        Commands::Logo => utils::print_logo(),
        Commands::Resources => {}
        Commands::Tasks => view::view_tasks().unwrap(),
    }
}
