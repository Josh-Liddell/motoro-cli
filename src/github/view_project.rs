use anyhow::Result;
use colored::Colorize;
use reqwest::blocking;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

const BASE_URL: &str = "https://api.github.com/graphql";
const ITEM_QUERY: &str = include_str!("../../queries/items.graphql");
const _PROJECT_QUERY: &str = include_str!("../../queries/projects.graphql");

// Struct to hold the data we need for a task.
// im matching to the response structure, there may be a better way idk
#[derive(Debug, Deserialize)]
struct Task {
    // id: String,
    #[serde(rename = "fieldValueByName")]
    field_value: TextField,
    status: NameField,
}

#[derive(Debug, Deserialize)]
struct TextField {
    text: String,
}

#[derive(Debug, Deserialize)]
struct NameField {
    name: String,
}

impl fmt::Display for Task {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(dest, "{}", self.field_value.text)
    }
}

pub struct MotoroTasks(Vec<Task>);

impl MotoroTasks {
    /// Query github for project board information using auth token
    /// https://docs.github.com/en/graphql/guides/forming-calls-with-graphql
    pub fn fetch(token: &str) -> Result<Self> {
        let client = blocking::Client::new();
        let motoro_project_id = "PVT_kwHOAJcjzs4BXDCe";

        let resp = client
            .post(BASE_URL)
            .bearer_auth(&token)
            .header("User-Agent", "motoro")
            .json(&json!({
                "query": ITEM_QUERY,
                "variables": {
                    "id": motoro_project_id
                }
            }))
            .send()?;

        // parse results
        let json: serde_json::Value = resp.json()?;
        if json.get("errors").is_some() {
            return Err(anyhow::anyhow!(
                "You may not have acess to the github project"
            ));
        }
        let json_cleaned = json.pointer("/data/node/items/nodes").unwrap();
        let tasks = serde_json::from_value(json_cleaned.clone())?;

        Ok(MotoroTasks(tasks))
    }

    pub fn view(&self) {
        for (heading, icon) in [
            ("Todo", '○'),
            ("In Progress", '►'),
            ("Blocked", '■'),
            ("Done", '✓'),
        ] {
            let mut it = self
                .0
                .iter()
                .filter(|t| t.status.name == heading)
                .peekable();
            if it.peek().is_some() {
                println!("{}", heading.green().bold());
                it.for_each(|t| println!(" {icon} {t}"));
            }
        }
    }
}
