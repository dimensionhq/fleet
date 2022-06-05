/*
 *
 *    Copyright 2021 Fleet Contributors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatCrateAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub crates: Vec<Crate>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
    pub name: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatFunctionAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub functions: Vec<Function>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "crate")]
    pub crate_field: String,
    pub name: String,
    pub size: i64,
}
