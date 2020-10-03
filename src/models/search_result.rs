use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
  url: String,
  score: f32,
}
