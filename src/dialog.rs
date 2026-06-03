use crate::{
    github::{self, MotoroTasks},
    utils::{self, GoogleDoc},
};
use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

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
