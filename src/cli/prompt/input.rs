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

#![allow(dead_code)]

use dialoguer::{
    console::{Key, Term},
    theme::{SimpleTheme, Theme},
};

use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
    io, iter,
    str::FromStr,
};

pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    pub fn set_prompts_reset_height(&mut self, val: bool) {
        self.prompts_reset_height = val;
    }

    pub fn term(&self) -> &Term {
        self.term
    }

    pub fn add_line(&mut self) {
        self.height += 1;
    }

    fn write_formatted_str<
        F: FnOnce(&mut TermThemeRenderer<'_>, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count();
        self.term.write_str(&buf)
    }

    fn write_formatted_line<
        F: FnOnce(&mut TermThemeRenderer<'_>, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        self.term.write_line(&buf)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut TermThemeRenderer<'_>, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    pub fn error(&mut self, err: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_error(buf, err))
    }

    pub fn confirm_prompt(&mut self, prompt: &str, default: Option<bool>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| this.theme.format_confirm_prompt(buf, prompt, default))
    }

    pub fn confirm_prompt_selection(&mut self, prompt: &str, sel: Option<bool>) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_confirm_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn input_prompt(&mut self, prompt: &str, default: Option<&str>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| this.theme.format_input_prompt(buf, prompt, default))
    }

    pub fn input_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_input_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn password_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            write!(buf, "\r")?;
            this.theme.format_password_prompt(buf, prompt)
        })
    }

    pub fn password_prompt_selection(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_password_prompt_selection(buf, prompt)
        })
    }

    pub fn select_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_select_prompt(buf, prompt))
    }

    pub fn select_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_select_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn select_prompt_item(&mut self, text: &str, active: bool) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme.format_select_prompt_item(buf, text, active)
        })
    }

    pub fn multi_select_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_multi_select_prompt(buf, prompt))
    }

    pub fn multi_select_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_multi_select_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn multi_select_prompt_item(
        &mut self,
        text: &str,
        checked: bool,
        active: bool,
    ) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_multi_select_prompt_item(buf, text, checked, active)
        })
    }

    pub fn sort_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_sort_prompt(buf, prompt))
    }

    pub fn sort_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_sort_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn sort_prompt_item(&mut self, text: &str, picked: bool, active: bool) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_sort_prompt_item(buf, text, picked, active)
        })
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> io::Result<()> {
        let mut new_height = self.height;
        //Check each item size, increment on finding an overflow
        for size in size_vec {
            if *size > self.term.size().1 as usize {
                new_height += 1;
            }
        }
        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}

pub trait Validator<T> {
    type Err: Debug + Display;

    /// Invoked with the value to validate.
    ///
    /// If this produces `Ok(())` then the value is used and parsed, if
    /// an error is returned validation fails with that error.
    fn validate(&mut self, input: &T) -> Result<(), Self::Err>;
}

impl<T, F: FnMut(&T) -> Result<(), E>, E: Debug + Display> Validator<T> for F {
    type Err = E;

    fn validate(&mut self, input: &T) -> Result<(), Self::Err> {
        self(input)
    }
}

pub struct Input<'a, T> {
    prompt: Cow<'a, str>,
    default: Option<T>,
    show_default: bool,
    initial_text: Option<Cow<'a, str>>,
    theme: &'a dyn Theme,
    permit_empty: bool,
    validator: Option<ValidatorFn<'a, T>>,
}

pub type ValidatorFn<'a, T> = Box<dyn FnMut(&T) -> Option<Cow<'a, str>> + 'a>;

impl<'a, T> Default for Input<'a, T>
where
    T: Clone + FromStr + Display,
    T::Err: Display + Debug,
{
    fn default() -> Input<'a, T> {
        Input::new()
    }
}

impl<'a, T> Input<'a, T>
where
    T: Clone + FromStr + Display,
    T::Err: Display + Debug,
{
    /// Creates an input prompt.
    pub fn new() -> Input<'a, T> {
        Input::with_theme(&SimpleTheme)
    }

    /// Creates an input prompt with a specific theme.
    pub fn with_theme(theme: &'a dyn Theme) -> Input<'a, T> {
        Input {
            prompt: "".into(),
            default: None,
            show_default: true,
            initial_text: None,
            theme,
            permit_empty: false,
            validator: None,
        }
    }

    /// Sets the input prompt.
    pub fn with_prompt<S: Into<Cow<'a, str>>>(&mut self, prompt: S) -> &mut Input<'a, T> {
        self.prompt = prompt.into();
        self
    }

    /// Sets initial text that user can accept or erase.
    pub fn with_initial_text<S: Into<Cow<'a, str>>>(&mut self, val: S) -> &mut Input<'a, T> {
        self.initial_text = Some(val.into());
        self
    }

    /// Sets a default.
    ///
    /// Out of the box the prompt does not have a default and will continue
    /// to display until the user inputs something and hits enter. If a default is set the user
    /// can instead accept the default with enter.
    pub fn default(&mut self, value: T) -> &mut Input<'a, T> {
        self.default = Some(value);
        self
    }

    /// Enables or disables an empty input
    ///
    /// By default, if there is no default value set for the input, the user must input a non-empty string.
    pub fn allow_empty(&mut self, val: bool) -> &mut Input<'a, T> {
        self.permit_empty = val;
        self
    }

    /// Disables or enables the default value display.
    ///
    /// The default behaviour is to append [`Self::default`] to the prompt to tell the
    /// user what is the default value.
    ///
    /// This method does not affect existance of default value, only its display in the prompt!
    pub fn show_default(&mut self, val: bool) -> &mut Input<'a, T> {
        self.show_default = val;
        self
    }

    /// Registers a validator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use dialoguer::Input;
    /// let mail: String = Input::new()
    ///     .with_prompt("Enter email")
    ///     .validate_with(|input: &String| -> Result<(), &str> {
    ///         if input.contains('@') {
    ///             Ok(())
    ///         } else {
    ///             Err("This is not a mail address")
    ///         }
    ///     })
    ///     .interact()
    ///     .unwrap();
    /// ```
    pub fn validate_with<V>(&mut self, mut validator: V) -> &mut Input<'a, T>
    where
        V: Validator<T> + 'a,
        T: 'a,
    {
        let mut old_validator_func = self.validator.take();

        self.validator = Some(Box::new(move |value: &T| -> Option<Cow<'a, str>> {
            if let Some(old) = old_validator_func.as_mut() {
                if let Some(err) = old(value) {
                    return Some(err);
                }
            }

            match validator.validate(value) {
                Ok(()) => None,
                Err(err) => Some(err.to_string().into()),
            }
        }));

        self
    }

    /// Enables the user to enter a printable ascii sequence and returns the result.
    ///
    /// Its difference from [`interact`](#method.interact) is that it only allows ascii characters for string,
    /// while [`interact`](#method.interact) allows virtually any character to be used e.g arrow keys.
    ///
    /// The dialog is rendered on stderr.
    pub fn interact_text(&mut self) -> io::Result<T> {
        self.interact_text_on(&Term::stderr())
    }

    /// Like [`interact_text`](#method.interact_text) but allows a specific terminal to be set.
    pub fn interact_text_on(&mut self, term: &Term) -> io::Result<T> {
        let mut render = TermThemeRenderer::new(term, self.theme);

        loop {
            let default_string = self.default.as_ref().map(ToString::to_string);

            render.input_prompt(
                &self.prompt,
                if self.show_default {
                    default_string.as_deref()
                } else {
                    None
                },
            )?;
            term.flush()?;

            // Read input by keystroke so that we can suppress ascii control characters
            if !term.features().is_attended() {
                return Ok("".to_owned().parse::<T>().unwrap());
            }

            let mut chars: Vec<char> = Vec::new();
            let mut position = 0;

            if let Some(initial) = self.initial_text.as_ref() {
                term.write_str(initial)?;
                chars = initial.chars().collect();
                position = chars.len();
            }

            loop {
                match term.read_key()? {
                    Key::Backspace if position > 0 => {
                        position -= 1;
                        chars.remove(position);
                        term.clear_chars(1)?;

                        let tail: String = chars[position..].iter().collect();

                        if !tail.is_empty() {
                            term.write_str(&tail)?;
                            term.move_cursor_left(tail.len())?;
                        }

                        term.flush()?;
                    }
                    Key::Char(chr) if !chr.is_ascii_control() => {
                        chars.insert(position, chr);
                        position += 1;
                        let tail: String =
                            iter::once(&chr).chain(chars[position..].iter()).collect();
                        term.write_str(&tail)?;
                        term.move_cursor_left(tail.len() - 1)?;
                        term.flush()?;
                    }
                    Key::ArrowLeft if position > 0 => {
                        term.move_cursor_left(1)?;
                        position -= 1;
                        term.flush()?;
                    }
                    Key::ArrowRight if position < chars.len() => {
                        term.move_cursor_right(1)?;
                        position += 1;
                        term.flush()?;
                    }
                    Key::Enter => break,
                    Key::Unknown => {}
                    _ => (),
                }
            }
            let input = chars.iter().collect::<String>();

            term.clear_line()?;
            render.clear()?;

            if chars.is_empty() {
                if let Some(ref default) = self.default {
                    render.input_prompt_selection(&self.prompt, &default.to_string())?;
                    term.flush()?;
                    return Ok(default.clone());
                } else if !self.permit_empty {
                    continue;
                }
            }

            match input.parse::<T>() {
                Ok(value) => {
                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator(&value) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    render.input_prompt_selection(&self.prompt, &input)?;
                    term.flush()?;

                    return Ok(value);
                }
                Err(err) => {
                    render.error(&err.to_string())?;
                    continue;
                }
            }
        }
    }

    /// Enables user interaction and returns the result.
    ///
    /// Allows any characters as input, including e.g arrow keys.
    /// Some of the keys might have undesired behavior.
    /// For more limited version, see [`interact_text`](#method.interact_text).
    ///
    /// If the user confirms the result is `true`, `false` otherwise.
    /// The dialog is rendered on stderr.
    pub fn interact(&mut self) -> io::Result<T> {
        self.interact_on(&Term::stderr())
    }

    /// Like [`interact`](#method.interact) but allows a specific terminal to be set.
    pub fn interact_on(&mut self, term: &Term) -> io::Result<T> {
        let mut render = TermThemeRenderer::new(term, self.theme);

        loop {
            let default_string = self.default.as_ref().map(ToString::to_string);

            render.input_prompt(
                &self.prompt,
                if self.show_default {
                    default_string.as_deref()
                } else {
                    None
                },
            )?;
            term.flush()?;

            let input = if let Some(initial_text) = self.initial_text.as_ref() {
                term.read_line_initial_text(initial_text)?
            } else {
                term.read_line()?
            };

            render.add_line();
            term.clear_line()?;
            render.clear()?;

            if input.is_empty() {
                if let Some(ref default) = self.default {
                    render.input_prompt_selection(&self.prompt, &default.to_string())?;
                    term.flush()?;
                    return Ok(default.clone());
                } else if !self.permit_empty {
                    continue;
                }
            }

            match input.parse::<T>() {
                Ok(value) => {
                    if let Some(ref mut validator) = self.validator {
                        if let Some(err) = validator(&value) {
                            render.error(&err)?;
                            continue;
                        }
                    }

                    render.input_prompt_selection(&self.prompt, &input)?;
                    term.flush()?;

                    return Ok(value);
                }
                Err(err) => {
                    render.error(&err.to_string())?;
                    continue;
                }
            }
        }
    }
}
