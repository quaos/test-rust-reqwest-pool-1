use log::{debug};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::errors::AppError;

static API_BASE_URL: &str = "https://customsearch.googleapis.com/customsearch/v1";

pub struct Client {
  engine_id: String,
  api_key: String,
  http_client: Arc<reqwest::Client>,
}

impl Client {
  pub fn new(
    engine_id: &str,
    api_key: &str,
    http_client: Arc<reqwest::Client>,
  ) -> Self {
    Self {
      engine_id: engine_id.to_string(),
      api_key: api_key.to_string(),
      http_client,
    }
  }

  pub async fn search(&self, keyword: &str) -> Result<SearchResult, AppError> {
    let resp = self.http_client
      .get(API_BASE_URL)
      .query(&[
        ("cx", self.engine_id.as_str()),
        ("key", self.api_key.as_str()),
        ("q", keyword),
      ])
      .send()
      .await?
      .error_for_status()?;

    let result = resp.json::<SearchResult>().await?;
    debug!("search for: {} => found {} results",
      keyword,
      result.search_information.total_results_as_i32()?
    );

    Ok(result)
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
  search_information: SearchInformationSection,
  items: Vec<SearchResultItem>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInformationSection {
  search_time: f32,
  total_results: String,
}

impl SearchInformationSection {
  pub fn total_results_as_i32(&self) -> Result<i32, AppError> {
    Ok(self.total_results.parse::<i32>()
      .map_err(|err| AppError::other(&err))?)
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultItem {
  title: String,
  link: String,
  snippet: String,
}
