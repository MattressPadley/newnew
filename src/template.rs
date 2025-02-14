use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub emoji: String,
    #[serde(default)]
    pub variables: Vec<TemplateVariable>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TemplateVariable {
    pub name: String,
    pub prompt: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub default: Option<String>,
    #[serde(rename = "if")]
    pub if_: Option<String>,
    pub if_condition: Option<String>,
    #[serde(rename = "if-not")]
    pub if_not: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Step {
    pub name: String,
    #[serde(default)]
    pub if_: Option<String>,
    #[serde(rename = "if")]
    pub if_condition: Option<String>,
    #[serde(rename = "if-not")]
    pub if_not: Option<String>,
    #[serde(default)]
    pub run: Option<String>,
    #[serde(default)]
    pub check: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub copy: Option<CopyStep>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CopyStep {
    pub from: String,
    pub to: String,
}

pub fn load_templates() -> io::Result<HashMap<String, Template>> {
    let template_dir = get_template_dir()?;
    let mut templates = HashMap::new();
    let mut had_errors = false;

    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
    }

    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("yml") {
            let template_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_yaml::from_str(&content) {
                        Ok(template) => {
                            templates.insert(template_name, template);
                        },
                        Err(e) => {
                            had_errors = true;
                            eprintln!("⚠️  Error parsing template '{}': {}", path.display(), e);
                            eprintln!("   This template will be skipped. Please check the YAML format.");
                            
                            // If it's specifically a variables format error, show migration help
                            if e.to_string().contains("variables: invalid type: map") {
                                eprintln!("\nℹ️  The template format has changed. Variables should now be a sequence.");
                                eprintln!("   Update your template from:");
                                eprintln!("   variables:");
                                eprintln!("     use_typescript:");
                                eprintln!("       prompt: \"Use TypeScript?\"");
                                eprintln!("\n   To:");
                                eprintln!("   variables:");
                                eprintln!("     - name: use_typescript");
                                eprintln!("       prompt: \"Use TypeScript?\"\n");
                            }
                        }
                    }
                },
                Err(e) => {
                    had_errors = true;
                    eprintln!("⚠️  Error reading template file '{}': {}", path.display(), e);
                }
            }
        }
    }

    if templates.is_empty() {
        if had_errors {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "No valid templates found due to parsing errors. Please fix the template files and try again."
            ));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No templates found. Try running with --examples to install example templates."
            ));
        }
    }

    Ok(templates)
}

pub fn copy_example_templates_if_needed(with_examples: bool) -> io::Result<()> {
    if !with_examples {
        return Ok(());
    }

    let template_dir = get_template_dir()?;
    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
    }
    
    copy_example_templates(&template_dir)
}

fn get_template_dir() -> io::Result<PathBuf> {
    dirs::home_dir()
        .map(|p| p.join(".config").join("newnew").join("templates"))
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory"
        ))
}

fn copy_example_templates(template_dir: &Path) -> io::Result<()> {
    println!("Copying templates to: {}", template_dir.display()); // Temporary debug
    let example_dir = Path::new("examples/templates");
    
    if !example_dir.exists() {
        println!("Example dir not found: {}", example_dir.display()); // Temporary debug
        return Ok(());
    }

    // First copy the YAML files
    for entry in fs::read_dir(example_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
            let dest = template_dir.join(path.file_name().unwrap());
            println!("Copying YAML: {} -> {}", path.display(), dest.display()); // Temporary debug
            fs::copy(&path, dest)?;
        }
    }

    // Then copy the template directories
    for entry in fs::read_dir(example_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name().unwrap();
            let dest_dir = template_dir.join(dir_name);
            println!("Copying dir: {} -> {}", path.display(), dest_dir.display()); // Temporary debug
            fs::create_dir_all(&dest_dir)?;
            copy_dir_recursive(&path, &dest_dir)?;
        }
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let source = entry.path();
        let dest = dst.join(source.file_name().unwrap());

        if ty.is_dir() {
            copy_dir_recursive(&source, &dest)?;
        } else {
            fs::copy(&source, &dest)?;
        }
    }
    Ok(())
} 