use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Cargo {
    pub build: Build,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Build {
    // add field rustc-wrapper
    #[serde(rename = "rustc-wrapper")]
    pub rustc_wrapper: String,
}
