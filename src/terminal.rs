use colored::Colorize;
use dialoguer::{
    Confirm, Input,
    console::{Style, style},
    theme::{ColorfulTheme, Theme},
};
use indicatif::ProgressBar;

pub fn print_done(msg: &str) {
    println!(" {} {}", "✓".bold().green(), msg)
}

pub fn print_error(msg: &str) {
    eprintln!(" {}: {}", "Error".red().bold(), msg);
}

pub fn print_progress(msg: &str, current: i8, total: i8) {
    println!("{} {}", format!("[{current}/{total}]").white().bold(), msg);
}

pub fn print_title(msg: &str) {
    println!("{}:", msg.bold());
}

fn get_dialog_theme() -> impl Theme {
    ColorfulTheme {
        prompt_prefix: style(" ?".to_string()).for_stdout().cyan(),
        success_prefix: style(" ✔".to_string()).for_stdout().green(),
        error_prefix: style(" ✘".to_string()).for_stderr().red(),
        defaults_style: Style::new().for_stdout().dim().white(),
        ..Default::default()
    }
}

pub fn ask_dialog(question: &str, default: bool, report: bool) -> bool {
    Confirm::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(default)
        .show_default(true)
        .report(report)
        .interact()
        .unwrap()
}

pub fn ask_string_dialog(question: &str, report: bool) -> String {
    Input::<String>::with_theme(&get_dialog_theme())
        .with_prompt(question)
        .default(String::new())
        .report(report)
        .interact_text()
        .unwrap()
}

pub fn generate_progress() -> ProgressBar {
    ProgressBar::new_spinner().with_style(
        indicatif::ProgressStyle::with_template(" {spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    )
}
