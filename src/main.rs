mod project;
mod templates;
mod utils;
mod github;
mod config;

use project::{ProjectConfig, ProjectTemplate, prompt_project_config};
use templates::{create_rust_project, create_python_project, create_platformio_project};
use utils::{prompt_input};
use github::{init_git_repository, create_github_repository};

use std::io;
use std::path::Path;
use std::fs;

fn main() {
    let config = prompt_project_config();
    
    match create_project(config) {
        Ok(_) => println!("✨ Project created successfully!"),
        Err(e) => eprintln!("❌ Error creating project: {}", e),
    }
}

fn create_project(config: ProjectConfig) -> io::Result<()> {
    let project_path = Path::new(&config.base_path).join(&config.name);
    fs::create_dir_all(&project_path)?;

    // Create project based on template
    match config.template {
        ProjectTemplate::RustCargo => create_rust_project(&project_path),
        ProjectTemplate::PythonPoetry => create_python_project(&project_path),
        ProjectTemplate::PlatformIOEmbed => create_platformio_project(&project_path),
    }?;

    // Initialize git repository
    init_git_repository(&project_path, &config.template)?;

    // Create and push to GitHub if requested
    if config.create_github_repo {
        create_github_repository(&project_path, &config.name)?;
    }

    Ok(())
}
