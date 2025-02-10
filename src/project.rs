use std::collections::HashMap;
use crate::utils::prompt_input;
use crate::config::Config;
use crate::template::{Template, load_templates};

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub template_name: String,
    pub template: Template,
    pub base_path: String,
    pub variables: HashMap<String, String>,
}

pub fn prompt_project_config() -> ProjectConfig {
    // Load config
    let config = Config::load().expect("Failed to load config");
    
    // Load templates
    let templates = load_templates().expect("Failed to load templates");
    
    // Display available templates
    println!("\nAvailable project templates:");
    for (name, template) in &templates {
        println!("  {} {} {}", template.emoji, name, template.description);
    }

    // Get template choice
    let template_name = loop {
        let choice = prompt_input("Choose template");
        if templates.contains_key(&choice) {
            break choice;
        }
        println!("Invalid template choice. Please try again.");
    };

    let template = templates.get(&template_name).unwrap().clone();
    
    // Get project name
    let name = prompt_input("Project name");
    
    // Collect variables from prompts
    let mut variables = HashMap::new();
    variables.insert("project_name".to_string(), name.clone());
    
    for (var_name, var) in &template.variables {
        // Skip if condition is not met
        if let Some(condition) = &var.if_condition {
            if !evaluate_condition(condition, &variables) {
                continue;
            }
        }

        let value = match var.type_.as_deref() {
            Some("boolean") => {
                let default = var.default.as_deref().unwrap_or("false");
                let prompt = format!("{} (y/N)", var.prompt);
                let response = prompt_input(&prompt).to_lowercase();
                (response.starts_with('y') || (response.is_empty() && default == "true")).to_string()
            },
            _ => {
                let default = var.default.as_deref().unwrap_or("");
                let prompt = if default.is_empty() {
                    var.prompt.clone()
                } else {
                    format!("{} (default: {})", var.prompt, default)
                };
                let response = prompt_input(&prompt);
                if response.is_empty() {
                    default.to_string()
                } else {
                    response
                }
            }
        };
        variables.insert(var_name.clone(), value);
    }

    ProjectConfig {
        name,
        template_name,
        template,
        base_path: config.settings.projects_dir
            .to_str()
            .expect("Invalid path")
            .to_string(),
        variables,
    }
}

fn evaluate_condition(condition: &str, variables: &HashMap<String, String>) -> bool {
    variables.get(condition)
        .map(|v| v == "true")
        .unwrap_or(false)
} 