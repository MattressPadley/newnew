use std::collections::HashMap;
use crate::utils::{prompt_input, prompt_select, prompt_confirm, prompt_multiselect};
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
    
    // Create formatted template options
    let template_options: Vec<String> = templates
        .iter()
        .map(|(name, template)| format!("{} {} {}", template.emoji, name, template.description))
        .collect();

    // Get template choice using select box
    let selected_template = prompt_select("Choose template", &template_options);
    let template_name = selected_template.split_whitespace().nth(1).unwrap().to_string();
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