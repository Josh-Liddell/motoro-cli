use colored::Colorize;
use reqwest::blocking;
use serde::Deserialize;
use std::io;
use std::io::Write;
use terminal_link::Link;

#[derive(Deserialize, Debug)]
pub struct AgencyResponse {
    results: Vec<Agency>,
}

#[derive(Deserialize, Debug)]
struct Agency {
    agency_id: u64,
    toptier_code: String,
    agency_name: String,
    congressional_justification_url: Option<String>,
}

const BASE_URL: &str = "https://api.usaspending.gov";

impl AgencyResponse {
    pub fn fetch() -> Result<Self, reqwest::Error> {
        blocking::get(format!("{}/api/v2/references/toptier_agencies/", BASE_URL))?.json()
    }

    pub fn pretty_print(&self) {
        let mut stdout = io::stdout().lock();
        for agency in &self.results {
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
}
