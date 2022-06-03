use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatCrateAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub crates: Vec<Crate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
    pub name: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatFunctionAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub functions: Vec<Function>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "crate")]
    pub crate_field: String,
    pub name: String,
    pub size: i64,
}
