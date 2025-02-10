use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::utils::check_command_exists;

pub fn create_rust_project(path: &Path) -> io::Result<()> {
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

pub fn create_python_project(path: &Path) -> io::Result<()> {
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

pub fn create_platformio_project(path: &Path) -> io::Result<()> {
    if !check_command_exists("pio") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "PlatformIO is not installed. Please install it from https://platformio.org"
        ));
    }

    println!("🤖 Initializing PlatformIO project...");
    
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