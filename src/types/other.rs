use serde::{Deserialize, Serialize};

use super::common::Color;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct ExtendedValue {
    pub number_value: Option<f64>,
    pub string_value: Option<String>,
    pub bool_value: Option<bool>,
    pub formula_value: Option<String>,
    // pub error_value: ErrorValue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct ColorStyle {
    pub rgb_color: Option<Color>,
    // pub theme_color: ThemeColorType,
}
