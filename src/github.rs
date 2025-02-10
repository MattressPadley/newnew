use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::project::ProjectTemplate;
use crate::utils::check_command_exists;

pub fn init_git_repository(path: &Path, template: &ProjectTemplate) -> io::Result<()> {
    if !check_command_exists("git") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Git is not installed. Please install Git from https://git-scm.com"
        ));
    }

    println!("📦 Initializing Git repository...");

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

pub fn create_github_repository(path: &Path, project_name: &str) -> io::Result<()> {
    if !check_command_exists("gh") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "GitHub CLI is not installed. Please install it from https://cli.github.com"
        ));
    }

    println!("🌐 Creating GitHub repository...");

    Command::new("gh")
        .args(["repo", "create", project_name, "--private", "--source", ".", "--remote", "origin"])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create GitHub repository: {}", e)
        ))?;

    println!("⬆️  Pushing to GitHub...");
    Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .current_dir(path)
        .status()
        .map_err(|e| io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to push to GitHub: {}", e)
        ))?;

    Ok(())
} 