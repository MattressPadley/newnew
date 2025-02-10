# newnew 🚀

A flexible project scaffolding tool that creates new projects from customizable YAML templates.

## Features ✨

- Template-driven project creation
- Conditional steps and file copying
- Variable substitution
- Command validation
- Git and GitHub integration
- Customizable project structure

## Installation 🔧

```bash
cargo install newnew
```

## Usage 💻

Simply run:
```bash
newnew
```

The tool will:
1. Show available templates
2. Prompt for template-specific variables
3. Create your project following the template steps

## Template System 📋

Templates are YAML files stored in `~/.config/newnew/templates/`. Each template defines:
- Basic information
- Variables to collect
- Steps to execute

### Template Structure

```yaml
name: Template Name
description: Template description
emoji: 🚀  # Emoji shown in template list

# Variables to collect from user
variables:
  variable_name:
    prompt: "Question to ask user?"
    type: boolean  # Optional, for yes/no questions
    default: false  # Optional default value
    if: another_variable  # Optional condition

# Steps to execute in sequence
steps:
  - name: Step Name  # Displayed during execution
    if: variable_name  # Optional condition
    check: command  # Optional command to check
    error: "Error if command missing"  # Optional error message
    run: command {with_variables}  # Command to run
    # OR
    copy:  # Copy and process template file
      from: template/file/path
      to: destination/path
```

### Variables

Variables are collected before execution and can be used in commands with `{variable_name}` syntax. Variables can be conditional based on other variables.

Supported variable types:
- `string` (default): Free text input
- `boolean`: Yes/no question with interactive selection
- `select`: Single choice from a list of options
- `multiselect`: Multiple choices from a list of options

Example variable definitions:
```yaml
variables:
  # Simple text input
  project_name:
    prompt: Project name
    default: my-project

  # Boolean yes/no selection
  use_docker:
    prompt: Add Docker support?
    type: boolean
    default: false

  # Single select from options
  project_type:
    prompt: Choose project type
    type: select
    options:
      - "Binary"
      - "Library"
      - "Both"

  # Multiple select from options
  dependencies:
    prompt: Select dependencies to include
    type: multiselect
    options:
      - "serde"
      - "tokio"
      - "clap"
      - "reqwest"
```

For `multiselect` variables, the selected values are joined with commas and can be accessed in commands or templates. For example:
```yaml
steps:
  - name: Add dependencies
    run: |
      cargo add {dependencies}
```

### Steps

Steps are executed in sequence. Each step can:
- Check for required commands
- Run shell commands
- Copy and process template files
- Be conditional based on variables

#### Command Steps
```yaml
steps:
  - name: Initialize git
    if: use_git  # Only run if use_git is true
    check: git  # Ensure git is installed
    error: "Git is not installed"
    run: |  # Multiline commands
      git init
      git add .
      git commit -m 'Initial commit'
```

#### File Steps
```yaml
steps:
  - name: Copy configuration
    if: use_config  # Optional condition
    copy:
      from: config.txt  # Relative to template directory
      to: .config  # Relative to project directory
```

### Custom Templates

Create your own templates in `~/.config/newnew/templates/`:

1. Create a YAML file (e.g., `custom.yml`)
2. Add template files in a matching directory (e.g., `custom/`)
3. Define variables and steps
4. Use the template with `newnew`

### Template Variables

These variables are always available:
- `project_name`: Name of the project
- `project_dir`: Full path to project directory

### Example Template

```yaml
name: Custom Template
description: Example custom template
emoji: 🎯

variables:
  use_docker:
    prompt: Add Docker support?
    type: boolean
    default: false
  language:
    prompt: Programming language
    default: typescript

steps:
  - name: Initialize project
    run: |
      mkdir -p src
      echo "# {project_name}" > README.md

  - name: Add Docker
    if: use_docker
    copy:
      from: docker/Dockerfile
      to: Dockerfile

  - name: Git setup
    if: use_github
    run: |
      git init
      git add .
      git commit -m 'Initial commit'
```

## Configuration 🔧

Configuration is stored in `~/.config/newnew/newnew.toml`:

```toml
[settings]
projects_dir = "~/Dev"  # Default project directory
```
