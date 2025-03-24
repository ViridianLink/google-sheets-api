use serde::{Deserialize, Serialize};

use super::{cells::CellFormat, common::Color, other::ExtendedValue};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct Sheet {
    pub properties: SheetProperties,
    pub data: Vec<GridData>,
    // pub merges: Vec<GridRange>,
    // pub conditional_formats: Vec<ConditionalFormatRule>,
    // pub filter_views: Vec<FilterView>,
    // pub protected_ranges: Vec<ProtectedRange>,
    // pub basic_filter: BasicFilter,
    // pub charts: Vec<EmbeddedChart>,
    // pub banded_ranges: Vec<BandedRange>,
    // pub developer_metadata: Vec<DeveloperMetadata>,
    // pub row_groups: Vec<DimensionGroup>,
    // pub column_groups: Vec<DimensionGroup>,
    // pub slicers: Vec<Slicer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct SheetProperties {
    pub sheet_id: u64,
    pub title: String,
    pub index: u64,
    // pub sheet_type: SheetType,
    // pub grid_properties: GridProperties,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub tab_color: Color,
    // pub tab_color_style: ColorStyle,
    #[serde(default)]
    pub right_to_left: bool,
    // pub data_source_sheet_properties: DataSourceSheetProperties,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct GridData {
    #[serde(default)]
    pub start_row: u64,
    #[serde(default)]
    pub start_column: u64,
    pub row_data: Vec<RowData>,
    // pub row_metadata: Vec<DimensionProperties>,
    // pub column_metadata: Vec<DimensionProperties>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct RowData {
    pub values: Vec<CellData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct CellData {
    pub user_entered_value: Option<ExtendedValue>,
    pub effective_value: Option<ExtendedValue>,
    pub formatted_value: Option<String>,
    pub user_entered_format: Option<CellFormat>,
    pub effective_format: CellFormat,
    pub hyperlink: Option<String>,
    #[serde(default)]
    pub note: String,
    // pub text_format_runs: Vec<TextFormatRun>,
    // pub data_validation: DataValidationRule,
    // pub pivot_table: PivotTable,
    // pub data_source_table: DataSourceTable,
    // pub data_source_formula: DataSourceFormula,
}
