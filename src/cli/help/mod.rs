use ansi_term::Colour::{Green, Yellow};

pub fn build_run_help_message() -> String {
    let help_menu = format!(
        r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
    fleet build [OPTIONS]

{}:
    -q, --quiet                     Do not print cargo log messages
        --bin [<NAME>]              Name of the bin target to run
        --example [<NAME>]          Name of the example target to run
    -p, --package [<SPEC>...]       Package with the target to run
    -v, --verbose                   Use verbose output (-vv very verbose/build.rs output)
    -j, --jobs <N>                  Number of parallel jobs, defaults to # of CPUs
        --color <WHEN>              Coloring: auto, always, never
        --keep-going                Do not abort the build as soon as there is an error (unstable)
        --frozen                    Require Cargo.lock and cache are up to date
    -r, --release                   Build artifacts in release mode, with optimizations
        --locked                    Require Cargo.lock is up to date
        --profile <PROFILE-NAME>    Build artifacts with the specified profile
        --features <FEATURES>       Space or comma separated list of features to activate
        --offline                   Run without accessing the network
        --all-features              Activate all available features
        --config <KEY=VALUE>        Override a configuration value (unstable)
        --no-default-features       Do not activate the `default` feature
    -Z <FLAG>                       Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                    details
        --target <TRIPLE>           Build for the target triple
        --target-dir <DIRECTORY>    Directory for all generated artifacts
        --manifest-path <PATH>      Path to Cargo.toml
        --message-format <FMT>      Error format
        --unit-graph                Output build graph in JSON (unstable)
        --ignore-rust-version       Ignore `rust-version` specification in packages
        --timings[=<FMTS>...]       Timing output formats (unstable) (comma separated): html, json
    -h, --help                      Print help information
"#,
        Green.paint("fleet"),
        env!("CARGO_PKG_VERSION"),
        Yellow.paint("USAGE"),
        Yellow.paint("OPTIONS"),
    );

    help_menu
}

pub fn build_build_help_message() -> String {
    let help_menu = format!(
        r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
    fleet build [OPTIONS]

{}:
    -q, --quiet                     Do not print cargo log messages
        --bin [<NAME>]              Name of the bin target to run
        --example [<NAME>]          Name of the example target to run
    -p, --package [<SPEC>...]       Package with the target to run
    -v, --verbose                   Use verbose output (-vv very verbose/build.rs output)
    -j, --jobs <N>                  Number of parallel jobs, defaults to # of CPUs
        --color <WHEN>              Coloring: auto, always, never
        --keep-going                Do not abort the build as soon as there is an error (unstable)
        --frozen                    Require Cargo.lock and cache are up to date
    -r, --release                   Build artifacts in release mode, with optimizations
        --locked                    Require Cargo.lock is up to date
        --profile <PROFILE-NAME>    Build artifacts with the specified profile
        --features <FEATURES>       Space or comma separated list of features to activate
        --offline                   Run without accessing the network
        --all-features              Activate all available features
        --config <KEY=VALUE>        Override a configuration value (unstable)
        --no-default-features       Do not activate the `default` feature
    -Z <FLAG>                       Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                    details
        --target <TRIPLE>           Build for the target triple
        --target-dir <DIRECTORY>    Directory for all generated artifacts
        --manifest-path <PATH>      Path to Cargo.toml
        --message-format <FMT>      Error format
        --unit-graph                Output build graph in JSON (unstable)
        --ignore-rust-version       Ignore `rust-version` specification in packages
        --timings[=<FMTS>...]       Timing output formats (unstable) (comma separated): html, json
    -h, --help                      Print help information
"#,
        Green.paint("fleet"),
        env!("CARGO_PKG_VERSION"),
        Yellow.paint("USAGE"),
        Yellow.paint("OPTIONS"),
    );
    help_menu
}
