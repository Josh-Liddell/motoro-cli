use crate::{resources, view_gh};
use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn home_dialoguer() -> Result<()> {
    let selections = &[
        "View the task board",
        "Useful links from telegram",
        "Something else...",
        "Something else...",
        "Something else...",
        "Something else...",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Make a selection")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => view_gh::view_tasks()?,
        1 => resources::fetch_links()?,
        _ => println!("idk!!"),
    }

    Ok(())
}
