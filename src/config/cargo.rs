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
    pub rustc_wrapper: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TartgetValues {
    pub rustflags: Vec<String>,
    pub linker: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "x86_64-unknown-linux-gnu")]
    pub linux: TartgetValues,
    #[serde(rename = "x86_64-pc-windows-msvc")]
    pub windows: TartgetValues,
    #[serde(rename = "target.x86_64-apple-darwin")]
    pub mac: TartgetValues,
}

pub fn add_rustc_wrapper_and_target_configs(path: &str, sccache_path: &str) {
    let config: ConfigToml = ConfigToml {
        build: Build {
            rustc_wrapper: sccache_path.to_string(),
        },
        target: Target {
            mac: TartgetValues {
                rustflags: vec![
                    String::from("-C"),
                    String::from("link-arg=-fuse-ld=/usr/local/bin/zld"),
                    String::from("-Zshare-generics=y"),
                    String::from("-Csplit-debuginfo=unpacked"),
                ],
                linker: None,
            },
            windows: TartgetValues {
                rustflags: vec![String::from("-Zshare-generics=y")],
                linker: Some(String::from("rust-lld.exe")),
            },
            linux: TartgetValues {
                rustflags: vec![
                    String::from("-Clink-arg=-fuse-ld=lld"),
                    String::from("-Zshare-generics=y"),
                ],
                linker: Some(String::from("/usr/bin/clang")),
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

    let mut toml_string = toml::to_string_pretty(&config).unwrap();

    std::fs::write(path, toml_string).unwrap();
}
