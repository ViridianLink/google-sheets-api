pub mod endpoints;
pub mod error;
pub mod sheets_client;
pub mod types;

pub use error::Error;
use error::Result;

pub use sheets_client::{SheetsClient, SheetsClientBuilder};

#[cfg(test)]
mod tests {
    use types::common::Color;

    use super::*;

    const ENDGAME_ANALYSIS_ID: &str = "1JM-0SlxVDAi-C6rGVlLxa-J1WGewEeL8Qvq4htWZHhY";

    #[tokio::test]
    async fn test() {
        let api_key = "";

        let client = SheetsClientBuilder::new(api_key).build().unwrap();

        let res = client.spreadsheet(ENDGAME_ANALYSIS_ID, true).await.unwrap();

        fn special_colour(color: &Color) -> bool {
            color.red == 0.0 && color.green == 1.0 && color.blue == 0.0
        }

        fn heavy_colour(color: &Color) -> bool {
            color.red == 0.6 && color.green == 0.0 && color.blue == 1.0
        }

        let tier_list_sheets = res
            .sheets
            .iter()
            .filter(|s| {
                !s.properties.hidden
                    && (special_colour(&s.properties.tab_color)
                        || heavy_colour(&s.properties.tab_color))
            })
            .collect::<Vec<_>>();

        let json = serde_json::to_string_pretty(&tier_list_sheets).unwrap();
        std::fs::write("endgame_analysis.json", json).unwrap();
    }
}
