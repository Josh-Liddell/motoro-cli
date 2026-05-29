use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

pub fn home_dialoguer() -> Result<()> {
    let selections = &[
        "View the task board",
        "Useful links from telegram",
        "Something else...",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Make a selection")
        // .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    // println!("Enjoy your {}!", selections[selection]);

    match selection {
        0 => crate::view_gh::view_tasks()?,
        1 => crate::resources::fetch_links()?,
        2 => println!("idk!!"),
        _ => {}
    }

    Ok(())
}
// let selection = Select::with_theme(&ColorfulTheme::default())
//     .with_prompt("Optionally pick your flavor")
//     .default(0)
//     .items(&selections[..])
//     .interact_opt()
//     .unwrap();

// if let Some(selection) = selection {
//     println!("Enjoy your {}!", selections[selection]);
// } else {
//     println!("You didn't select anything!");
// }

// let selection = Select::with_theme(&ColorfulTheme::default())
//     .with_prompt("Pick your flavor, hint it might be on the second page")
//     .default(0)
//     .max_length(2)
//     .items(&selections[..])
//     .interact()
//     .unwrap();

// println!("Enjoy your {}!", selections[selection]);
