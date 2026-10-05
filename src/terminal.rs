use std::io::IsTerminal;

use anyhow::{Result, anyhow};
use dialoguer::{
    Confirm, Input,
    console::{Style, style},
    theme::{ColorfulTheme, Theme},
};
use indicatif::ProgressBar;

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        eprintln!(" {}: {}", "Error".bright_red().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_done {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!(" {} {}", "✓".bold().bright_green(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_title {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!("{}:", format!($($arg)*).bold());
    }};
}

#[macro_export]
macro_rules! print_action_add {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!("{} {}", "==>".bright_green().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_action_run {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!("{} {}", "=>>".bright_blue().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_action_remove {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!("{} {}", "=>>".bright_red().bold(), format_args!($($arg)*));
    }};
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

pub fn ask_dialog(question: &str, default: bool, report: bool) -> Result<bool> {
    Confirm::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(default)
        .show_default(true)
        .report(report)
        .interact()
        .map_err(|_| anyhow!("CLI interaction failed"))
}

pub fn ask_string_dialog(question: &str, report: bool) -> Result<String> {
    Input::<String>::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(String::new())
        .report(report)
        .interact_text()
        .map_err(|_| anyhow!("CLI interaction failed"))
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
