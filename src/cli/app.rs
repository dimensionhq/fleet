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

use std::{env::current_dir, path::PathBuf};

use crate::config::FleetConfig;

pub struct App {
    pub config: FleetConfig,
    pub current_dir: PathBuf,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {

    ///Creates a new app
    ///
    /// # Panics
    /// Can panic if current directory is not found (eg. doesn't exist or invalid perms)
    #[must_use]
    pub fn new() -> Self {
        let current_dir = current_dir().expect("Unable to find current directory for app!");
        // let config = FleetConfig::new();
        // check
        // let config = config.run_config();

        //wasn't using the self argument anyway, so unsure of use
        Self {
            config: FleetConfig::run_config(),
            current_dir,
        }
    }
}
