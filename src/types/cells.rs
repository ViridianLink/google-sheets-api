use serde::{Deserialize, Serialize};

use super::other::ColorStyle;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct CellFormat {
    // pub number_format: NumberFormat,
    // pub background_color: Color,
    pub background_color_style: Option<ColorStyle>,
    // pub borders: Borders,
    // pub padding: Padding,
    // pub horizontal_alignment: HorizontalAlign,
    // pub vertical_alignment: VerticalAlign,
    // pub wrap_strategy: WrapStrategy,
    // pub text_direction: TextDirection,
    // pub text_format: TextFormat,
    // pub hyperlink_display_type: HyperlinkDisplayType,
    // pub text_rotation: TextRotation,
}
