# NewNew 🚀

A command-line tool that helps you quickly scaffold new projects with common templates and best practices.

## Features ✨

- Creates projects with predefined templates:
  - 🦀 Rust projects (using Cargo)
  - 🐍 Python projects (using Poetry)
  - 🤖 Embedded projects (using PlatformIO)
- Automatically initializes Git repository
- Creates standard project structure
- Sets up common development tools and configurations

## Prerequisites 📋

Before using this tool, ensure you have the following installed:

- Git (https://git-scm.com)
- Rust and Cargo (https://rustup.rs) - for creating Rust projects
- Poetry (https://python-poetry.org) - for creating Python projects
- PlatformIO (https://platformio.org) - for creating embedded projects

## Installation 🔧

1. Install using Homebrew:
   ```bash
   brew tap mattresspadley/tap
   brew install newnew
   ```

2. Or, build from source:
   1. Clone this repository
   2. Install the binary:
      ```bash
      cargo install --path .
      ```

## Usage 💻

1. Run the tool:
   ```bash
   newnew
   ```

2. Follow the interactive prompts:
   - Enter your project name
   - Choose a project template:
     1. Rust (Cargo)
     2. Python (Poetry)
     3. PlatformIO (Embedded)

3. The tool will:
   - Create a new directory in `~/Dev/[project-name]`
   - Initialize the chosen project template
   - Set up Git repository with initial commit
   - Configure development tools

## Project Templates 📁

### Rust (Cargo)
- Initializes a new Cargo project
- Sets up standard Rust project structure

### Python (Poetry)
- Creates `src` and `tests` directories
- Initializes Poetry for dependency management
- Configures `.gitignore` for Python projects

### PlatformIO (Embedded)
- Initializes a new PlatformIO project
- Default configuration for ESP32
- Creates basic Arduino-style template code
