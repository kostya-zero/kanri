use std::io::IsTerminal;

use anyhow::Result;
use colored::Colorize;
use dialoguer::{
    Confirm, Input, Select,
    console::{Style, style},
    theme::{ColorfulTheme, Theme},
};
use indicatif::ProgressBar;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminalError {
    #[error("CLI interaction failed.")]
    InteractionFailed,
}

pub fn print_error(msg: &str) {
    eprintln!(" {}: {msg}", "Error".bright_red().bold());
}

pub fn print_done(msg: &str) {
    println!(" {} {msg}", "✓".bold().bright_green())
}

pub fn print_title(msg: &str) {
    println!("{}:", msg.bold());
}

fn get_dialog_theme() -> impl Theme {
    ColorfulTheme {
        prompt_prefix: style(" ?".to_string()).for_stdout().cyan(),
        success_prefix: style(" ✓".to_string()).for_stdout().green(),
        error_prefix: style(" ✘".to_string()).for_stderr().red(),
        defaults_style: Style::new().for_stdout().dim().white(),
        ..Default::default()
    }
}

pub fn ask_dialog(question: &str, default: bool, report: bool) -> Result<bool, TerminalError> {
    Confirm::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(default)
        .show_default(true)
        .report(report)
        .interact()
        .map_err(|_| TerminalError::InteractionFailed)
}

pub fn ask_string_dialog(question: &str, report: bool) -> Result<String, TerminalError> {
    Input::<String>::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(String::new())
        .report(report)
        .interact_text()
        .map_err(|_| TerminalError::InteractionFailed)
}

pub fn ask_select(items: &Vec<String>, report: bool) -> Result<usize, TerminalError> {
    Select::with_theme(&get_dialog_theme())
        .with_prompt("Which template to use?")
        .default(0)
        .items(items)
        .report(report)
        .interact()
        .map_err(|_| TerminalError::InteractionFailed)
}

pub fn is_terminal() -> bool {
    std::io::stdin().is_terminal()
}

pub fn generate_progress() -> ProgressBar {
    ProgressBar::new_spinner().with_style(
        indicatif::ProgressStyle::with_template(" {spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    )
}
