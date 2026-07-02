mod display;
mod github;
mod usaspending;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use github::MotoroTasks;
use std::process::Command;
// use usaspending::AgencyResponse;
use dirs;
use usaspending::USAspendingClient;
use utils::GoogleDoc;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Cli {
    // #[arg(short, long)]
    // example: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Default)]
enum Commands {
    /// View the project task board on github
    Tasks,
    /// Print out the links to learning resources (from telegram)
    Resources,
    /// Julia code functionality (see motoro julia -h)
    Julia {
        /// run sample julia code
        #[arg(short, long)]
        example: bool,
    },
    /// Interact with JuliaHub
    JH {
        #[arg(value_enum)]
        job: Jobs,
    },
    /// Print the motoro logo
    #[default]
    Logo,
    /// Print out agencies with relevant URLs as provided by usaspending.gov
    Usaspending {
        /// list agencies
        #[arg(short, long)]
        agencies: bool,
        #[arg(short, long)]
        geo: bool,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Jobs {
    Example,
    Thisjob,
    Thatjob,
}

pub fn run() -> Result<()> {
    let args = Cli::parse();

    match args.command.unwrap_or_default() {
        Commands::Usaspending { agencies, geo } => {
            let client = USAspendingClient::new();

            if agencies {
                // fetch agency data and print
                let agencies = client.agencies()?;
                display::print_agencies(&agencies);
            } else if geo {
                // fetch spending data and output to csv
                let data = client.geo_spending()?;
                if let Some(path) = dirs::download_dir() {
                    utils::to_csv(&data, path.join("spending_data.csv"))?;
                    println!("Data exported to csv in your downloads folder");
                } else {
                    eprintln!("No downloads folder found");
                }
            }
        }
        Commands::Tasks => {
            let token = github::get_access_token()?;
            let tasks = MotoroTasks::fetch(&token)?;
            tasks.view();
        }
        Commands::Resources => {
            let doc = GoogleDoc::fetch_txt("1wKpnGjoNIqRh2UWR8bdTrWC1JEffrh4bRURK_0k8GIk")?;
            doc.render_terminal_links();
        }
        Commands::Julia { example } => {
            if example {
                let julia = include_str!("../julia/example.jl");
                Command::new("julia")
                    .arg("-e")
                    .arg(julia)
                    .status()
                    .expect("issue executing the julia command");
            } else {
                utils::start_julia_repl()?;
            }
        }
        Commands::JH { job } => {
            println!("You selected this job: {:?}", job);

            match job {
                Jobs::Example => {
                    let mut jl = Command::new("julia");
                    let code = include_str!("../julia/job.jl");

                    jl.arg("-e")
                        .arg(code)
                        .status()
                        .expect("issue executing the julia command");
                }
                _ => {}
            }
            // let mut jl = Command::new("Julia");
            // let code = include_str!("../julia/job.jl");

            // jl.arg("-e")
            //     .arg(code)
            //     .status()
            //     .expect("issue executing the julia command");

            // IMPORTANT
            // first ensure that pkg is installed, if not add it
            // then ensure the authentication has happened, if not do it
            // let check_package = jl
            //     .arg("-e")
            //     .arg("using JuliaHub")
            //     .stdout(Stdio::null())
            //     .stderr(Stdio::null())
            //     .status();

            // let is_installed = check_package.map(|s| s.success()).unwrap_or(false);

            // if !is_installed {
            //     println!("JuliaHub package not detected. Installing it now...");

            //     Command::new("julia")
            //         .arg("-e")
            //         .arg("using Pkg; Pkg.add(\"JuliaHub\")")
            //         .status()
            //         .expect("Failed to install JuliaHub package");

            //     println!("JuliaHub installed successfully!");
            // }
        }
        Commands::Logo => {
            // start julia runtime on another thread? idk
            // IF we want rust to be able to execute julia stuff
            display::print_logo();
            display::home_dialoguer()?;
        }
    }

    Ok(())
}
