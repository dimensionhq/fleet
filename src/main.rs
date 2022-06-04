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

//! 
//! Fleet is the blazing fast build tool for Rust. Compiling with Fleet is up-to 5x faster than with cargo.
//!
//! Note: Since fleet is in the beta phase, it might not be completely stable yet. Feel free to open any issues or bug reports at issues.
//!
//! Note: As of now fleet only supports rustc nightly
//! 
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use cli::CLI;

pub mod cli;
pub mod commands;
pub mod config;
pub mod utils;

/// Entrypoint to the CLI application
fn main() {
    CLI::run();
}
