use crate::utils::prompt_input;
use crate::config::Config;

#[derive(Debug, Clone)]
pub enum ProjectTemplate {
    RustCargo,
    PythonPoetry,
    PlatformIOEmbed,
}

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub template: ProjectTemplate,
    pub base_path: String,
    pub create_github_repo: bool,
}

pub fn prompt_project_config() -> ProjectConfig {
    // Load config
    let config = Config::load().expect("Failed to load config");

    // Get project name
    let name = prompt_input("Project name");

    // Display enabled template options
    println!("\nAvailable project templates:");
    let mut valid_choices = Vec::new();

    if config.settings.enabled_templates.contains(&"rust".to_string()) {
        println!("  1. 🦀 Rust (Cargo)");
        valid_choices.push(("1".to_string(), ProjectTemplate::RustCargo));
    }
    if config.settings.enabled_templates.contains(&"python".to_string()) {
        let choice = (valid_choices.len() + 1).to_string();
        println!("  {}. 🐍 Python (Poetry)", choice);
        valid_choices.push((choice, ProjectTemplate::PythonPoetry));
    }
    if config.settings.enabled_templates.contains(&"platformio".to_string()) {
        let choice = (valid_choices.len() + 1).to_string();
        println!("  {}. 🤖 PlatformIO (Embedded)", choice);
        valid_choices.push((choice, ProjectTemplate::PlatformIOEmbed));
    }

    let choice = prompt_input(&format!("Choose template (1-{})", valid_choices.len()));
    let template = valid_choices
        .iter()
        .find(|(c, _)| c == &choice)
        .map(|(_, t)| t.clone())
        .expect("Invalid template choice");

    // Use projects_dir from config
    let base_path = config.settings.projects_dir
        .to_str()
        .expect("Invalid path")
        .to_string();

    // Add prompt for GitHub repo creation
    let create_github_repo = prompt_input("Create GitHub repository? (y/N)")
        .to_lowercase()
        .starts_with('y');

    ProjectConfig {
        name,
        template,
        base_path,
        create_github_repo,
    }
} 