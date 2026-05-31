use crate::auth;
use anyhow::Result;
use colored::Colorize;
use reqwest::blocking;
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
fn get_tasks() -> Result<Vec<Task>> {
    // make api request on behalf of user using the access token
    let client = blocking::Client::new();
    let token = auth::get_access_token()?;
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

    Ok(tasks)
}

pub fn view_tasks() -> Result<()> {
    let tasks = get_tasks()?;
    // print the tasks
    for (heading, icon) in [
        ("Todo", '○'),
        ("In Progress", '►'),
        ("Blocked", '■'),
        ("Done", '✓'),
    ] {
        println!("{}", heading.green().bold());
        tasks
            .iter()
            .filter(|t| t.status.name == heading)
            .for_each(|t| println!(" {icon} {t}"));
    }
    // fix to not show header if no tasks

    Ok(())
}

// let project_id = "PVT_kwHOAJcjzs4BMQw-";

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
