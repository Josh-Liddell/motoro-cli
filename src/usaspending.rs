// mod agency;

use anyhow::{Result, anyhow};
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use serde_json::json;

// this should just be used for making get and post requests to the api
// methods that return data from the api
pub struct USAspendingClient {
    http_client: blocking::Client,
    base_url: String,
}

impl USAspendingClient {
    pub fn new() -> Self {
        Self {
            http_client: blocking::Client::new(),
            base_url: "https://api.usaspending.gov".to_string(),
        }
    }

    // a generic function that something that has this trait or something

    pub fn geo_spending(&self) -> Result<Vec<StateInfo>> {
        let url = format!("{}/api/v2/search/spending_by_geography/", self.base_url);
        let filters = json!({
            "filters": {
                "keywords": ["transport"]
            },
            "scope": "place_of_performance",
            "geo_layer": "state"
        });

        let resp = self.http_client.post(&url).json(&filters).send()?;

        if resp.status().is_success() {
            Ok(resp.json::<GeoSpendResponse>()?.results)
        } else {
            // println!("{}", resp.text()?);
            Err(anyhow!("There was an issue sending the request"))
        }

        // Ok(resp.results)
    }

    pub fn agencies(&self) -> Result<Vec<Agency>> {
        let url = format!("{}/api/v2/references/toptier_agencies/", self.base_url);
        let resp = self
            .http_client
            .get(&url)
            .send()?
            .json::<AgencyResponse>()?;

        Ok(resp.results)
    }
}

// TYPES

// Agencies
#[derive(Deserialize, Debug)]
struct AgencyResponse {
    results: Vec<Agency>,
}

#[derive(Deserialize, Debug)]
pub struct Agency {
    pub agency_id: u64,
    pub toptier_code: String,
    pub agency_name: String,
    pub congressional_justification_url: Option<String>,
}

// Geo Spending
#[derive(Deserialize)]
struct GeoSpendResponse {
    results: Vec<StateInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct StateInfo {
    pub shape_code: String,
    pub display_name: Option<String>,
    pub aggregated_amount: f64,
    pub population: Option<i32>,
    pub per_capita: Option<f64>,
}
