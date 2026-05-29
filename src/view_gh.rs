use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

const BASE_URL: &str = "https://api.github.com/graphql";
const ITEM_QUERY: &str = include_str!("../queries/items.graphql");
const _PROJECT_QUERY: &str = include_str!("../queries/projects.graphql");

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

/// Query github for project board information using auth token
/// https://docs.github.com/en/graphql/guides/forming-calls-with-graphql
pub fn view_tasks() -> Result<()> {
    // make api requests using the token
    let client = reqwest::blocking::Client::new();
    let token = crate::auth::get_access_token()?;
    // let project_id = "PVT_kwHOAJcjzs4BMQw-";
    let project_id = "PVT_kwHOAJcjzs4BXDCe";
    let query = json!({
        "query": ITEM_QUERY,
        "variables": {
            "id": project_id
        }
    });

    // send query
    let resp = client
        .post(BASE_URL)
        .bearer_auth(&token)
        .header("User-Agent", "motoro")
        .json(&query)
        .send()?;

    // parse results
    let json: serde_json::Value = resp.json()?;
    if json.get("errors").is_some() {
        return Err(anyhow::anyhow!(
            "You may not have acess to the github project"
        ));
    }
    let json_cleaned = json.pointer("/data/node/items/nodes").unwrap();
    let tasks_list: Vec<Task> = serde_json::from_value(json_cleaned.clone())?;

    // print the tasks
    for (heading, icon) in [("Todo", "○"), ("In Progress", "►"), ("Done", "✓")] {
        println!("{}", heading.green().bold());
        tasks_list
            .iter()
            .filter(|t| t.status.name == heading)
            .for_each(|t| println!(" {icon} {t}"));
    }

    Ok(())
}

// pub fn view_projects() -> Result<()> {
//     // make api requests using the token
//     let client = reqwest::blocking::Client::new();
//     let token = crate::auth::get_access_token()?;
//     let query = json!({
//         "query": PROJECT_QUERY,
//         "variables": {
//             "owner": "broughtj"
//         }
//     });

//     // send query
//     let resp = client
//         .post(BASE_URL)
//         .bearer_auth(&token)
//         .header("User-Agent", "motoro")
//         .json(&query)
//         .send()?;

//     let content = resp.text()?;
//     println!("{}", content);

//     Ok(())
// }
