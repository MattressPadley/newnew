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
    pub variables: HashMap<String, Variable>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Variable {
    pub prompt: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub default: Option<String>,
    #[serde(rename = "if")]
    pub if_: Option<String>,
    pub if_condition: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Step {
    pub name: String,
    #[serde(default)]
    pub if_: Option<String>,
    #[serde(rename = "if")]
    pub if_condition: Option<String>,
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

    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
    }

    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("yml") {
            let template: Template = serde_yaml::from_str(&fs::read_to_string(&path)?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap()
                .to_string();
                
            templates.insert(name, template);
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