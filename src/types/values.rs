use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    pub range: String,
    // pub major_dimension: Dimension,
    pub values: Vec<Vec<String>>,
}