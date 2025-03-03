use serde::{Deserialize, Serialize};

use super::sheet::Sheet;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct Spreadsheet {
    pub spreadsheet_id: String,
    // pub properties: SpreadsheetProperties,
    pub sheets: Vec<Sheet>,
    // pub named_ranges: Vec<NamedRange>,
    pub spreadsheet_url: String,
    // pub developer_metadata: Vec<DeveloperMetadata>,
    // pub data_sources: Vec<DataSource>,
    // pub data_source_schedules: Vec<DataSourceRefreshSchedule>,
}
