use anyhow::Result;
use colored::Colorize;
use dirs;
use std::thread;
use std::time::Duration;
use std::{env, path::PathBuf, process::Command};

/// Returns a path to the Motoro julia project
/// If the env variable was not set it downloads the project
fn motoro_path() -> Result<PathBuf> {
    match env::var_os("MOTORO_JULIA_PATH").map(PathBuf::from) {
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

/// Prints the motoro logo
pub fn print_logo() {
    let greens = [
        (140, 255, 160),
        (110, 245, 170),
        (85, 230, 185),
        (60, 210, 200),
        (45, 185, 210),
        (35, 160, 220),
        (25, 130, 230),
        (20, 100, 240),
    ];

    let logo = r"
$$\      $$\            $$\
$$$\    $$$ |           $$ |
$$$$\  $$$$ | $$$$$$\ $$$$$$\    $$$$$$\   $$$$$$\   $$$$$$\
$$\$$\$$ $$ |$$  __$$\\_$$  _|  $$  __$$\ $$  __$$\ $$  __$$\
$$ \$$$  $$ |$$ /  $$ | $$ |    $$ /  $$ |$$ |  \__|$$ /  $$ |
$$ |\$  /$$ |$$ |  $$ | $$ |$$\ $$ |  $$ |$$ |      $$ |  $$ |
$$ | \_/ $$ |\$$$$$$  | \$$$$  |\$$$$$$  |$$ |      \$$$$$$  |
\__|     \__| \______/   \____/  \______/ \__|       \______/";

    let subtitle = "Computational Options Pricing";

    for (line, color) in logo.lines().skip(1).zip(greens) {
        println!("{}", line.custom_color(color).bold());
        thread::sleep(Duration::from_secs_f32(0.25));
    }

    let width = logo.lines().map(|l| l.len()).max().unwrap();
    println!("\n{:^1$}\n\n", subtitle, width);

    thread::sleep(Duration::from_secs_f32(0.5));
}
