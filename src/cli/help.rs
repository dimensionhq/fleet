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

use colored::Colorize;

pub fn build_help() -> String {
    format!(
        r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
    fleet build [OPTIONS]

{}:
    -q, --quiet                     Do not print cargo log messages
    -p, --package [<SPEC>]          Package to build (see `cargo help pkgid`)
        --workspace                 Build all packages in the workspace
        --exclude <SPEC>            Exclude packages from the build
    -v, --verbose                   Use verbose output (-vv very verbose/build.rs output)
        --all                       Alias for --workspace (deprecated)
        --color <WHEN>              Coloring: auto, always, never
    -j, --jobs <N>                  Number of parallel jobs, defaults to # of CPUs
        --frozen                    Require Cargo.lock and cache are up to date
        --keep-going                Do not abort the build as soon as there is an error (unstable)
        --lib                       Build only this package's library
        --locked                    Require Cargo.lock is up to date
        --bin [<NAME>]              Build only the specified binary
        --offline                   Run without accessing the network
        --bins                      Build all binaries
        --config <KEY=VALUE>        Override a configuration value (unstable)
        --example [<NAME>]          Build only the specified example
    -Z <FLAG>                       Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for
                                    details
        --examples                  Build all examples
        --test [<NAME>]             Build only the specified test target
        --tests                     Build all tests
        --bench [<NAME>]            Build only the specified bench target
        --benches                   Build all benches
        --all-targets               Build all targets
    -r, --release                   Build artifacts in release mode, with optimizations
        --profile <PROFILE-NAME>    Build artifacts with the specified profile
        --features <FEATURES>       Space or comma separated list of features to activate
        --all-features              Activate all available features
        --no-default-features       Do not activate the `default` feature
        --target <TRIPLE>           Build for the target triple
        --target-dir <DIRECTORY>    Directory for all generated artifacts
        --out-dir <PATH>            Copy final artifacts to this directory (unstable)
        --manifest-path <PATH>      Path to Cargo.toml
        --ignore-rust-version       Ignore `rust-version` specification in packages
        --message-format <FMT>      Error format
        --build-plan                Output the build plan in JSON (unstable)
        --unit-graph                Output build graph in JSON (unstable)
        --future-incompat-report    Outputs a future incompatibility report at the end of the build
        --timings[=<FMTS>...]       Timing output formats (unstable) (comma separated): html, json
    -h, --help                      Print help information
"#,
        "fleet".green(),
        env!("CARGO_PKG_VERSION").green(),
        "USAGE".yellow(),
        "OPTIONS".yellow()
    )
}

pub fn run_help() -> String {
    format!(
        r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
fleet run [OPTIONS]

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
        "fleet".green(),
        env!("CARGO_PKG_VERSION").green(),
        "USAGE".yellow(),
        "OPTIONS".yellow()
    )
}
