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
    /// Adds files to myapp
    Add {
        name: Option<String>,
    },
    /// Hello from the tools
    Print,
    Run,
}

pub fn run() {
    let args = Args::parse();

    match &args.command {
        Commands::Add { name } => {
            println!("'add' was used, name is: {name:?}");
        }
        Commands::Print => crate::utils::print_logo(),
        Commands::Run => {
            let _x = 56;
            println!("running")
        }
    }
}
