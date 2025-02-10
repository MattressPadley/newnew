use std::io::{self, Write};
use std::process::Command;
use dialoguer::{theme::ColorfulTheme, Select, Confirm, MultiSelect};

pub fn prompt_input(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

pub fn prompt_select(prompt: &str, options: &[String]) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    options[selection].clone()
}

pub fn prompt_confirm(prompt: &str, default: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .unwrap()
}

pub fn prompt_multiselect(prompt: &str, options: &[String]) -> Vec<String> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        .interact()
        .unwrap();

    selections.into_iter()
        .map(|i| options[i].clone())
        .collect()
}

pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
} 