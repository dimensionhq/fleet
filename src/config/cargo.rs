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

use std::process::exit;

use ansi_term::Colour::Red;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigToml {
    pub build: Build,
    #[serde(rename = "target")]
    pub target: Target,

    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileValues {
    #[serde(rename = "opt-level")]
    pub opt_level: u8,
    pub debug: u8,
    pub incremental: bool,
    #[serde(rename = "codegen-units")]
    pub codegen_units: u16,
    #[serde(rename = "split-debuginfo")]
    pub split: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub dev: ProfileValues,
    pub release: ProfileValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    #[serde(rename = "rustc-wrapper")]
    pub rustc_wrapper: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetValues {
    pub rustflags: Vec<String>,
    pub linker: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "x86_64-unknown-linux-gnu")]
    pub linux: TargetValues,
    #[serde(rename = "x86_64-pc-windows-msvc")]
    pub windows: TargetValues,
    #[serde(rename = "x86_64-apple-darwin")]
    pub mac: TargetValues,
}

/// # Panics
/// Can panic if cannot prettify config
pub fn add_rustc_wrapper_and_target_configs(
    path: &str,
    sccache_path: Option<String>,
    clang_path: Option<String>,
    lld_path: Option<String>,
) {
    let config: ConfigToml = ConfigToml {
        build: Build {
            rustc_wrapper: sccache_path,
        },
        target: Target {
            mac: TargetValues {
                rustflags: vec![
                    String::from("-C"),
                    String::from("link-arg=-fuse-ld=/usr/local/bin/zld"),
                    String::from("-Zshare-generics=y"),
                    String::from("-Csplit-debuginfo=unpacked"),
                ],
                linker: None,
            },
            windows: TargetValues {
                rustflags: vec![String::from("-Zshare-generics=y")],
                linker: lld_path,
            },
            linux: TargetValues {
                rustflags: vec![
                    String::from("-Clink-arg=-fuse-ld=lld"),
                    String::from("-Zshare-generics=y"),
                ],
                linker: clang_path,
            },
        },
        profile: Profile {
            release: ProfileValues {
                split: Some(String::from("...")),

                opt_level: 3,
                debug: 0,
                incremental: false,
                codegen_units: 256,
            },
            dev: ProfileValues {
                codegen_units: 512,
                debug: 2,
                split: None,
                incremental: true,
                opt_level: 0,
            },
        },
    };

    let toml_string = toml::to_string_pretty(&config).expect("Cannot prettify config");

    std::fs::write(path, toml_string).unwrap_or_else(|err| {
        println!(
            "{}: failed to write configuration: {}",
            Red.paint("error"),
            err
        );

        exit(1);
    });

    println!("📝 Generated Fleet Config");
}
