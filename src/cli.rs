use crate::{
    dialog,
    github::{self, MotoroTasks},
    utils::{self, GoogleDoc},
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
pub struct Cli {
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

impl Cli {
    pub fn run() -> Result<()> {
        let args = Self::parse();

        match args.command {
            Some(Commands::Tasks) => {
                let token = github::get_access_token()?;
                let tasks = MotoroTasks::fetch(&token)?;
                tasks.view();
            }
            Some(Commands::Resources) => {
                let doc = GoogleDoc::fetch_txt("1wKpnGjoNIqRh2UWR8bdTrWC1JEffrh4bRURK_0k8GIk")?;
                doc.render_terminal_links();
            }
            Some(Commands::Julia) => utils::start_julia_repl()?,
            None => {
                // start julia runtime on another thread? idk
                // IF we want rust to be able to execute julia stuff
                utils::print_logo();
                dialog::home_dialoguer()?;
            }
        }

        Ok(())
    }
}
