use std::collections::HashMap;
use crate::utils::{prompt_input, prompt_select, prompt_confirm, prompt_multiselect};
use crate::config::Config;
use crate::template::{Template, load_templates, copy_example_templates_if_needed};

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub template_name: String,
    pub template: Template,
    pub base_path: String,
    pub variables: HashMap<String, String>,
}

pub fn prompt_project_config(with_examples: bool, target_dir: Option<String>) -> ProjectConfig {
    // Load config
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("⚠️  Failed to load config: {}", e);
            eprintln!("   Using default configuration.");
            Config::default()
        }
    };
    
    // Copy example templates if flag is set
    if let Err(e) = copy_example_templates_if_needed(with_examples) {
        eprintln!("⚠️  Failed to copy example templates: {}", e);
    }
    
    // Load templates
    let templates = match load_templates() {
        Ok(templates) => templates,
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    };
    
    // Create formatted template options
    let template_options: Vec<String> = templates
        .iter()
        .map(|(name, template)| format!("{} {} {}", template.emoji, name, template.description))
        .collect();

    if template_options.is_empty() {
        eprintln!("No templates found. Try running with --examples to install example templates.");
        std::process::exit(1);
    }

    // Get template choice using select box
    let selected_template = prompt_select("Choose template", &template_options);
    let template_name = selected_template.split_whitespace().nth(1).unwrap().to_string();
    let template = templates.get(&template_name).unwrap().clone();
    
    // Get project name
    let name = prompt_input("Project name");
    
    // Collect variables from prompts
    let mut variables = HashMap::new();
    variables.insert("project_name".to_string(), name.clone());
    
    // Process variables in order
    for var in &template.variables {
        // Check both if and if-not conditions
        if let Some(condition) = &var.if_condition {
            if !evaluate_condition(condition, &variables) {
                println!("↪ Skipping variable '{}': condition '{}' not met", var.name, condition);
                continue;
            }
        }
        if let Some(condition) = &var.if_not {
            if evaluate_condition(condition, &variables) {
                println!("↪ Skipping variable '{}': if-not condition '{}' not met", var.name, condition);
                continue;
            }
        }

        let value = match var.type_.as_deref() {
            Some("boolean") => {
                let default = var.default.as_deref().unwrap_or("false") == "true";
                prompt_confirm(&var.prompt, default).to_string()
            },
            Some("multiselect") => {
                if let Some(options) = &var.options {
                    let selections = prompt_multiselect(&var.prompt, options);
                    selections.join(",")
                } else {
                    prompt_input(&var.prompt)
                }
            },
            Some("select") => {
                if let Some(options) = &var.options {
                    prompt_select(&var.prompt, options)
                } else {
                    prompt_input(&var.prompt)
                }
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
        variables.insert(var.name.clone(), value);
    }

    ProjectConfig {
        name,
        template_name,
        template,
        base_path: target_dir.unwrap_or_else(|| config.settings.projects_dir
            .to_str()
            .expect("Invalid path")
            .to_string()),
        variables,
    }
}

fn evaluate_condition(condition: &str, variables: &HashMap<String, String>) -> bool {
    // Check if it's a negated condition
    if condition.starts_with('!') {
        let actual_condition = &condition[1..];
        return variables.get(actual_condition)
            .map(|v| v != "true")
            .unwrap_or(true);
    }
    
    variables.get(condition)
        .map(|v| v == "true")
        .unwrap_or(false)
} 