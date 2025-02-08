use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use dirs;

enum ProjectType {
    Rust,
    Python,
    PlatformIO,
}

struct ProjectConfig {
    name: String,
    project_type: ProjectType,
    path: String,
}

fn main() {
    let config = get_project_config();
    match create_project(config) {
        Ok(_) => println!("Project created successfully!"),
        Err(e) => eprintln!("Error creating project: {}", e),
        
    }
}

fn get_project_config() -> ProjectConfig {
    // Get project name
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    // Get project type
    println!("Select project type:");
    println!("1. Rust");
    println!("2. Python");
    println!("3. PlatformIO");
    print!("Enter your choice (1-3): ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let project_type = match choice.trim() {
        "1" => ProjectType::Rust,
        "2" => ProjectType::Python,
        "3" => ProjectType::PlatformIO,
        _ => panic!("Invalid choice"),
    };

    // Always use ~/Dev as the project path
    let path = dirs::home_dir()
        .expect("Could not find home directory")
        .join("Dev")
        .to_str()
        .expect("Invalid path")
        .to_string();

    ProjectConfig {
        name,
        project_type,
        path,
    }
}

fn create_project(config: ProjectConfig) -> io::Result<()> {
    let project_path = Path::new(&config.path).join(&config.name);
    fs::create_dir_all(&project_path)?;

    match config.project_type {
        ProjectType::Rust => create_rust_project(&project_path),
        ProjectType::Python => create_python_project(&project_path),
        ProjectType::PlatformIO => create_platformio_project(&project_path),
    }
}

fn create_rust_project(path: &Path) -> io::Result<()> {
    Command::new("cargo")
        .arg("init")
        .arg(path)
        .status()
        .expect("Failed to initialize Rust project");
    Ok(())
}

fn create_python_project(path: &Path) -> io::Result<()> {
    // Create basic Python project structure
    fs::create_dir(path.join("src"))?;
    fs::create_dir(path.join("tests"))?;
    
    // Initialize Poetry
    Command::new("poetry")
        .arg("init")
        .arg("--name").arg(path.file_name().unwrap().to_str().unwrap())
        .arg("--author").arg("Matt Hadley <hello@matthadley.me")
        .current_dir(path)
        .status()
        .expect("Failed to initialize Poetry project");

    Command::new("poetry")
        .arg("install")
        .current_dir(path)
        .status()
        .expect("Failed to install Poetry dependencies");

    Ok(())
}

fn create_platformio_project(path: &Path) -> io::Result<()> {
    // Initialize PlatformIO project
    Command::new("pio")
        .arg("project")
        .arg("init")
        .arg("--board").arg("esp32dev") // Default to ESP32, can be made configurable
        .current_dir(path)
        .status()
        .expect("Failed to initialize PlatformIO project");

    // Create a basic main.cpp file if it doesn't exist
    let src_path = path.join("src/main.cpp");
    if !src_path.exists() {
        fs::write(
            src_path,
            b"#include <Arduino.h>\n\nvoid setup() {\n    Serial.begin(115200);\n}\n\nvoid loop() {\n    Serial.println(\"Hello, World!\");\n    delay(1000);\n}\n"
        )?;
    }

    Ok(())
}
