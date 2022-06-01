/*
    Copyright 2021 Volt Contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use crate::cli::prompt::input;

use dialoguer::{console, theme::ColorfulTheme};
use std::{borrow::Cow, io::Result};

/// Prompt that returns `true` or `false` (as strings)
#[derive(Debug)]
pub struct Confirm<'i> {
    /// Message for the prompt
    pub message: Cow<'i, str>,

    /// Default value for the prompt is `true`
    pub default: bool,
    // TODO: Validation
    // #[structopt(short, long)]
    // /// Command to validate the submitted value
    // validate: Option<String>,
}

impl<'i> Confirm<'i> {
    pub fn run(&self) -> Result<bool> {
        let theme = ColorfulTheme {
            defaults_style: console::Style::new(),
            prompt_style: console::Style::new(),
            prompt_prefix: console::style(String::from("?")).yellow().bright(),
            prompt_suffix: console::style(String::from(">")).blue().dim(),
            success_prefix: console::style(String::from("✔")).green().bright(),
            success_suffix: console::style(String::from("·")).blue().dim(),
            error_prefix: console::style(String::from("❌")).bright().red(),
            error_style: console::Style::new(),
            hint_style: console::Style::new(),
            values_style: console::Style::new(),
            active_item_style: console::Style::new(),
            inactive_item_style: console::Style::new(),
            active_item_prefix: console::style(String::from("✔")).bright().green(),
            inactive_item_prefix: console::style(String::from(" ")),
            checked_item_prefix: console::style(String::from("")),
            unchecked_item_prefix: console::style(String::from("")),
            picked_item_prefix: console::style(String::from("")),
            unpicked_item_prefix: console::style(String::from("")),
            inline_selections: false,
        };

        let value = dialoguer::Confirm::with_theme(&theme)
            .with_prompt(self.message.clone().into_owned())
            .default(self.default)
            .interact()?;

        Ok(value)
    }
}

/// Prompt that takes user input and returns a string.
#[derive(Debug)]
pub struct Input<'i> {
    /// Message for the prompt
    pub message: Cow<'i, str>,

    /// Default value for the prompt
    pub default: Option<Cow<'i, str>>,

    /// Allow empty input. Conflicts with `default`
    pub allow_empty: bool,
}

impl Input<'_> {
    /// # Panics
    /// Can panic if cannot get self.default as ref
    pub fn run(&self) -> Result<String> {
        let theme = ColorfulTheme {
            defaults_style: console::Style::new(),
            prompt_style: console::Style::new(),
            prompt_prefix: console::style(String::from("?")).yellow().bright(),
            prompt_suffix: console::style(String::from(">")).blue().dim(),
            success_prefix: console::style(String::from("✔")).green().bright(),
            success_suffix: console::style(String::from("·")).blue().dim(),
            error_prefix: console::style(String::from("❌")).bright().red(),
            error_style: console::Style::new(),
            hint_style: console::Style::new(),
            values_style: console::Style::new(),
            active_item_style: console::Style::new(),
            inactive_item_style: console::Style::new(),
            active_item_prefix: console::style(String::from("✔")).bright().green(),
            inactive_item_prefix: console::style(String::from(" ")),
            checked_item_prefix: console::style(String::from("")),
            unchecked_item_prefix: console::style(String::from("")),
            picked_item_prefix: console::style(String::from("")),
            unpicked_item_prefix: console::style(String::from("")),
            inline_selections: false,
        };

        let mut input = input::Input::<String>::with_theme(&theme);

        input
            .with_prompt(self.message.clone())
            .allow_empty(self.allow_empty);

        if self.default.is_some() {
            input.default(
                self.default
                    .as_ref()
                    .expect("cannot get self.default asref")
                    .to_string(),
            );
        }

        let value = input.interact_text()?;

        Ok(value)
    }
}
/// Prompt that takes user input, hides it from the terminal, and returns a string
#[derive(Debug)]
pub struct Secret<'i> {
    /// Message for the prompt
    pub message: Cow<'i, str>,

    /// Enable confirmation prompt with this message
    pub confirm: Option<Cow<'i, str>>,

    /// Error message when secrets doesn't match during confirmation
    pub error: Option<Cow<'i, str>>,

    /// Allow empty secret
    pub allow_empty: bool,
}

impl<'i> Secret<'i> {
    #[allow(dead_code)]
    pub fn run(&self) -> Result<String> {
        let theme = ColorfulTheme::default();
        let mut input = dialoguer::Password::with_theme(&theme);

        input
            .with_prompt(self.message.clone())
            .allow_empty_password(self.allow_empty);

        if let (Some(confirm), Some(error)) = (&self.confirm, &self.error) {
            input.with_confirmation(confirm.clone().into_owned(), error.clone().into_owned());
        }

        let value = input.interact()?;

        Ok(value)
    }
}

/// Prompt that allows the user to select from a list of options
#[derive(Debug)]
pub struct Select<'i> {
    /// Message for the prompt
    pub message: Cow<'i, str>,

    /// Enables paging. Uses your terminal size
    pub paged: bool,

    /// Specify number of the item that will be selected by default
    pub selected: Option<usize>,

    /// Items that can be selected
    pub items: Vec<Cow<'i, str>>,
}

impl<'i> Select<'i> {
    pub fn run(&self) -> Result<usize> {
        let item_len = self.items.len();

        if item_len == 0 {
            return Ok(0);
        }

        let theme = ColorfulTheme {
            defaults_style: console::Style::new(),
            prompt_style: console::Style::new().bold(),
            prompt_prefix: console::style(String::from("?")).yellow().bright(),
            prompt_suffix: console::style(String::from(">")).blue().dim(),
            success_prefix: console::style(String::from("✔")).green().bright(),
            success_suffix: console::style(String::from("·")).blue().dim(),
            error_prefix: console::style(String::from("❌")).bright().red(),
            error_style: console::Style::new(),
            hint_style: console::Style::new().bold(),
            values_style: console::Style::new(),
            active_item_style: console::Style::new(),
            inactive_item_style: console::Style::new(),
            active_item_prefix: console::style(String::from("✔")).bright().green(),
            inactive_item_prefix: console::style(String::from(" ")),
            checked_item_prefix: console::style(String::from("")),
            unchecked_item_prefix: console::style(String::from("")),
            picked_item_prefix: console::style(String::from("")),
            unpicked_item_prefix: console::style(String::from("")),
            inline_selections: false,
        };

        let mut input = dialoguer::Select::with_theme(&theme);

        input
            .with_prompt(self.message.clone())
            //.paged(self.paged)
            .items(&self.items);

        if let Some(selected) = self.selected {
            input.default(selected - 1);
        }

        input.interact()
    }
}
