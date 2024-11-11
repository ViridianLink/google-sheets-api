use url::Url;

use crate::types::spreadsheet::Spreadsheet;
use crate::{Result, SheetsClient};

impl SheetsClient {
    pub async fn spreadsheet(&self, spreadsheet_id: &str, grid_data: bool) -> Result<Spreadsheet> {
        let mut url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}",
            spreadsheet_id
        )
        .parse::<Url>()?;

        if grid_data {
            url.query_pairs_mut().append_pair("includeGridData", "true");
        }

        self.get::<Spreadsheet>(url).await
    }

    pub async fn spreadsheet_values(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<Spreadsheet> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id, range
        )
        .parse::<Url>()?;

        self.get::<Spreadsheet>(url).await
    }
}
