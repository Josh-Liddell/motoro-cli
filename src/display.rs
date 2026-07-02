use crate::{
    github::{self, MotoroTasks},
    usaspending::Agency,
    utils::{self, GoogleDoc},
};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Select, theme::ColorfulTheme};
use std::io::{self, Write};
use std::{thread, time::Duration};
use terminal_link::Link;

pub fn print_agencies(agencies: &[Agency]) {
    let mut stdout = io::stdout().lock();
    for agency in agencies {
        let display_name = match &agency.congressional_justification_url {
            Some(url) => Link::new(&agency.agency_name, url).to_string(),
            None => agency.agency_name.clone(),
        };

        writeln!(
            stdout,
            "{} (id: {}, toptier_code: {})",
            display_name.magenta().bold(),
            agency.agency_id,
            agency.toptier_code
        )
        .expect("Failed to write line");
    }
}

pub fn home_dialoguer() -> Result<()> {
    let selections = &[
        "View the task board",
        "Useful links from telegram",
        "Work in julia on motoro", // starts the julia repl in the project.
        "Execute julia code via here",
        "quit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Make a selection")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            let token = github::get_access_token()?;
            let tasks = MotoroTasks::fetch(&token)?;
            tasks.view();
        }
        1 => {
            let doc = GoogleDoc::fetch_txt("1wKpnGjoNIqRh2UWR8bdTrWC1JEffrh4bRURK_0k8GIk")?;
            doc.render_terminal_links();
        }
        2 => utils::start_julia_repl()?,
        3 => println!("Depends on what you want it to do!"),
        4 => std::process::exit(0),
        _ => println!("idk!!"),
    }

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
