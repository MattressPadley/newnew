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

    // Display template options
    println!("\nAvailable project templates:");
    println!("  1. 🦀 Rust (Cargo)");
    println!("  2. 🐍 Python (Poetry)");
    println!("  3. 🤖 PlatformIO (Embedded)");
    
    let template = match prompt_input("Choose template (1-3)").as_str() {
        "1" => ProjectTemplate::RustCargo,
        "2" => ProjectTemplate::PythonPoetry,
        "3" => ProjectTemplate::PlatformIOEmbed,
        _ => panic!("Invalid template choice"),
    };

    // Use projects_dir from config
    let base_path = config.projects_dir
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