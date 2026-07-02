#![allow(unused)]

use anyhow::Result;
use colored::Colorize;
use dirs;
use reqwest::blocking;
use serde::Serialize;
use std::path::Path;
use std::{env, path::PathBuf, process::Command, thread, time::Duration};

pub fn to_csv<T, P>(collection: &[T], path: P) -> Result<()>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let mut wtr = csv::Writer::from_path(path)?;

    for record in collection {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}

pub struct GoogleDoc {
    pub id: String,
    pub content: String,
}

impl GoogleDoc {
    pub fn fetch_txt(doc_id: &str) -> Result<Self> {
        let url = format!(
            "https://docs.google.com/document/d/{}/export?format=txt",
            doc_id
        );
        let resp = blocking::get(url)?;

        if resp.status().is_success() {
            let content = resp.text()?;
            Ok(Self {
                id: doc_id.to_string(),
                content,
            })
        } else {
            Err(anyhow::anyhow!("Failed to fetch Google Doc: {}", doc_id))
        }
    }

    pub fn render_terminal_links(&self) {
        println!("{}", "Links from the telegram conversation".blue().bold());
        println!("{}", "Hold cmd/ctrl and click to open".magenta().bold());
        println!("\n{}", self.content);
    }
}

/// Returns a path to the Motoro julia project
/// If the env variable was not set it downloads the project
fn motoro_path() -> Result<PathBuf> {
    match env::var_os("MOTORO_PATH").map(PathBuf::from) {
        Some(p) if p.is_dir() => Ok(p),
        _ => {
            let path = dirs::cache_dir().unwrap().join("motoro/");
            let mut git = Command::new("git");

            if path.exists() {
                // make sure code is to date
                println!("Ensuring code is up to date...");
                git.args(["-C", path.to_str().unwrap(), "pull"]).status()?;
            } else {
                // get the code
                println!("motor path not set cloning the repo...");
                git.args(["clone", "https://github.com/broughtj/Motoro.jl.git"])
                    .arg(&path)
                    .status()?;
            }
            Ok(path)
        }
    }
}

pub fn start_julia_repl() -> Result<()> {
    let path = motoro_path()?.join("Motoro"); // change this join if they refactor the project
    println!("Starting julia...");
    std::process::Command::new("julia")
        .arg(format!("--project={}", path.display()))
        .arg("--banner=no")
        .status()?;

    Ok(())
}
