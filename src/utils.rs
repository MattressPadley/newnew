use std::io::{self, Write};
use std::process::Command;

pub fn prompt_input(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

pub fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
} 