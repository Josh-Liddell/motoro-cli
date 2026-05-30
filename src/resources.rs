use anyhow::Result;
use colored::Colorize;
use reqwest::blocking;

/// Prints out text from a google doc
pub fn fetch_links() -> Result<()> {
    let doc_id = "1wKpnGjoNIqRh2UWR8bdTrWC1JEffrh4bRURK_0k8GIk";
    let url = format!(
        "https://docs.google.com/document/d/{}/export?format=txt",
        doc_id
    );
    let response = blocking::get(url)?;

    if response.status().is_success() {
        let content = response.text()?;
        println!(
            "{}\n\n{}",
            "Links from the telegram conversation".blue().bold(),
            "Hold command (mac) or ctrl (windows) and click a link to open it (on ghostty)"
                .magenta()
                .bold()
        );
        println!("{}", content);
        Ok(())
    } else {
        Err(anyhow::anyhow!("There was an error fetching the document"))
    }
}
