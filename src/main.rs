mod project;
mod template;
mod utils;
mod config;

use clap::Parser;
use project::{ProjectConfig, prompt_project_config};
use utils::check_command_exists;
use std::io;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Install example templates
    #[arg(long)]
    examples: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = prompt_project_config(cli.examples);
    
    match create_project(config) {
        Ok(_) => println!("✨ Project created successfully!"),
        Err(e) => eprintln!("❌ Error creating project: {}", e),
    }
}

fn create_project(config: ProjectConfig) -> io::Result<()> {
    // Convert ~ to absolute home directory path
    let base_path = if config.base_path.starts_with('~') {
        dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?
            .join(config.base_path.strip_prefix("~/").unwrap_or(&config.base_path))
    } else {
        PathBuf::from(config.base_path)
    };
    
    // Create base directory if it doesn't exist
    fs::create_dir_all(&base_path)?;
    
    let project_path = base_path.join(&config.name);
    fs::create_dir_all(&project_path)?;  // Create project directory immediately

    // Create a new variables HashMap with project_dir added
    let mut variables = config.variables.clone();
    variables.insert("project_dir".to_string(), project_path.to_string_lossy().into_owned());

    // Process steps in sequence
    for step in &config.template.steps {
        println!("⚡ {}", step.name);

        // Check if step should be run based on condition
        if let Some(condition) = &step.if_condition {
            if !evaluate_condition(condition, &variables) {
                continue;
            }
        }

        // Check if required command exists
        if let Some(check_cmd) = &step.check {
            if !check_command_exists(check_cmd) {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    step.error.as_deref().unwrap_or(&format!("{} is not installed", check_cmd))
                ));
            }
        }

        // Handle copy step
        if let Some(copy) = &step.copy {
            let template_dir = dirs::home_dir()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?
                .join(".config")
                .join("newnew")
                .join("templates");

            let source = template_dir.join(&copy.from);
            let dest = project_path.join(&copy.to);

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = fs::read_to_string(&source)
                .map_err(|e| io::Error::new(
                    e.kind(),
                    format!("Failed to read template file '{}': {}", source.display(), e)
                ))?;
            let expanded_content = expand_variables(&content, &variables);
            fs::write(&dest, expanded_content)
                .map_err(|e| io::Error::new(
                    e.kind(),
                    format!("Failed to write file '{}': {}", dest.display(), e)
                ))?;
        }

        // Handle run command
        if let Some(run_cmd) = &step.run {
            // Handle multiline commands (commands with |)
            for cmd in run_cmd.split('\n') {  // Split by newlines instead of |
                let cmd = cmd.trim();
                if cmd.is_empty() {
                    continue;
                }
                
                let expanded_cmd = expand_variables(cmd, &variables);
                
                // Split command preserving quoted strings
                let mut parts = Vec::new();
                let mut current = String::new();
                let mut in_quotes = false;
                let mut quote_char = None;
                
                for c in expanded_cmd.chars() {
                    match c {
                        '\'' | '"' => {
                            match quote_char {
                                None => {
                                    quote_char = Some(c);
                                    in_quotes = true;
                                }
                                Some(q) if q == c => {
                                    quote_char = None;
                                    in_quotes = false;
                                }
                                _ => current.push(c),
                            }
                        }
                        ' ' if !in_quotes => {
                            if !current.is_empty() {
                                parts.push(current);
                                current = String::new();
                            }
                        },
                        _ => current.push(c),
                    }
                }
                if !current.is_empty() {
                    parts.push(current);
                }

                let command = parts.first().unwrap();
                let args = &parts[1..];

                Command::new(command)
                    .args(args)
                    .current_dir(&project_path)
                    .status()
                    .map_err(|e| io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to run command '{}': {}", expanded_cmd, e)
                    ))?;
            }
        }
    }

    Ok(())
}

fn evaluate_condition(condition: &str, variables: &HashMap<String, String>) -> bool {
    variables.get(condition)
        .map(|v| v == "true")
        .unwrap_or(false)
}

fn expand_variables(text: &str, variables: &HashMap<String, String>) -> String {
    let mut result = text.to_string();
    for (key, value) in variables {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}
