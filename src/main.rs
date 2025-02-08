use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use dirs;

// Make enum variants more descriptive and add derive traits
#[derive(Debug, Clone)]
enum ProjectTemplate {
    RustCargo,
    PythonPoetry,
    PlatformIOEmbed,
}

// Add documentation and derive traits
#[derive(Debug)]
struct ProjectConfig {
    name: String,
    template: ProjectTemplate,
    base_path: String,
}

fn main() {
    let config = prompt_project_config();
    
    match create_project(config) {
        Ok(_) => println!("✨ Project created successfully!"),
        Err(e) => eprintln!("❌ Error creating project: {}", e),
    }
}

fn prompt_project_config() -> ProjectConfig {
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

    // Use ~/Dev as the base project path
    let base_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join("Dev")
        .to_str()
        .expect("Invalid path")
        .to_string();

    ProjectConfig {
        name,
        template,
        base_path,
    }
}

// Helper function to reduce code duplication in prompts
fn prompt_input(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

fn check_command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
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
    init_git_repository(&project_path, &config.template)
}

fn init_git_repository(path: &Path, template: &ProjectTemplate) -> io::Result<()> {
    if !check_command_exists("git") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Git is not installed. Please install Git from https://git-scm.com"
        ));
    }

    println!("📦 Initializing Git repository...");

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize git repository: {}", e)
        ))?;

    // Create .gitignore only for Python projects
    if let ProjectTemplate::PythonPoetry = template {
        let gitignore_content = r#"
__pycache__/
*.py[cod]
*$py.class
.Python
.env
.venv
env/
venv/
ENV/
dist/
build/
*.egg-info/
.pytest_cache/
.coverage
htmlcov/
"#;

        fs::write(
            path.join(".gitignore"),
            gitignore_content.trim_start()
        )?;
    }

    // Create initial commit
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to stage files: {}", e)
        ))?;

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create initial commit: {}", e)
        ))?;

    Ok(())
}

fn create_rust_project(path: &Path) -> io::Result<()> {
    if !check_command_exists("cargo") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Cargo is not installed. Please install Rust from https://rustup.rs"
        ));
    }

    println!("🦀 Initializing Rust project...");
    
    Command::new("cargo")
        .arg("init")
        .arg(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize Rust project: {}", e)
        ))?;
        
    Ok(())
}

fn create_python_project(path: &Path) -> io::Result<()> {
    if !check_command_exists("poetry") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Poetry is not installed. Please install it from https://python-poetry.org"
        ));
    }

    println!("🐍 Initializing Python project...");
    
    // Create standard Python project structure
    fs::create_dir(path.join("src"))?;
    fs::create_dir(path.join("tests"))?;
    
    // Initialize and configure Poetry
    let project_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
        
    Command::new("poetry")
        .args([
            "init",
            "--name", project_name,
            "--author", "Matt Hadley <hello@matthadley.me>",
            // "--version", "0.0.1",
            "--no-interaction",
        ])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize Poetry project: {}", e)
        ))?;

    Command::new("poetry")
        .arg("install")
        .arg("--no-root")
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to install Poetry dependencies: {}", e)
        ))?;

    Ok(())
}

fn create_platformio_project(path: &Path) -> io::Result<()> {
    if !check_command_exists("pio") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "PlatformIO is not installed. Please install it from https://platformio.org"
        ));
    }

    println!("🤖 Initializing PlatformIO project...");
    
    // Initialize PlatformIO project
    Command::new("pio")
        .args([
            "project", "init",
            "--board", "esp32dev", // Default to ESP32
        ])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize PlatformIO project: {}", e)
        ))?;

    // Create default main.cpp if it doesn't exist
    let src_path = path.join("src/main.cpp");
    if !src_path.exists() {
        let template = r#"#include <Arduino.h>

void setup() {
    Serial.begin(115200);
}

void loop() {
    Serial.println("Hello, World!");
    delay(1000);
}
"#;
        fs::write(src_path, template)?;
    }

    Ok(())
}
