use anyhow::Result;
use serde_json::json;

const BASE_URL: &str = "https://api.github.com/graphql";

/// Query github for project board information using auth token
/// https://docs.github.com/en/graphql/guides/forming-calls-with-graphql
pub fn view_tasks() -> Result<()> {
    // make api requests using the token
    let client = reqwest::blocking::Client::new();
    let token = super::auth::get_access_token()?;

    // let query = json!({
    //     "query": r#"
    //         query($owner: String!) {
    //             user(login: $owner) {
    //                 projectsV2(first: 10) {
    //                     nodes {
    //                         id
    //                         title
    //                         shortDescription
    //                     }
    //                 }
    //             }
    //         }
    //     "#,
    //     "variables": {
    //         "owner": "broughtj" // <-- Put the target name here
    //     }
    // });
    let project_id = "PVT_kwHOAJcjzs4BMQw-";

    let query = json!({
        "query": r#"
            query($id: ID!) {
                node(id: $id) {
                    ... on ProjectV2 {
                        items(first: 20) {
                            nodes {
                                id
                                # 1. This gets the text displayed on the card, even if it's a closed repo issue
                                fieldValueByName(name: "Title") {
                                    ... on ProjectV2ItemFieldTextValue {
                                        text
                                    }
                                }
                                # 2. This gets the Kanban column name (e.g., "In Progress", "Done")
                                status: fieldValueByName(name: "Status") {
                                    ... on ProjectV2ItemFieldSingleSelectValue {
                                        name
                                    }
                                }
                            }
                        }
                    }
                }
            }
        "#,
        "variables": {
            "id": project_id
        }
    });

    let resp = client
        .post(BASE_URL)
        .bearer_auth(&token) // <-- Attaches "Authorization: Bearer <token>"
        .header("User-Agent", "motoro")
        .json(&query)
        .send()?;

    let response_text = resp.text()?;
    println!("API Response:\n{}", response_text);

    Ok(())
}
