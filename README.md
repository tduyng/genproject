
# genproject

## Project Generator CLI

A CLI tool to generate starter project templates for various programming environments (e.g., Node.js, NestJS, Rust, Deno). It supports custom project configurations via flags, with an optional interactive mode for guided setup.

---

## Features

- Generate project templates for multiple types of projects (Node.js, NestJS, Rust, Deno etc.).
- Configure project options through command-line flags.
- Use an interactive mode for step-by-step guided setup.
- Optionally include linters (e.g., ESLint, Biome) for project configurations.

---

## Installation

To install and use this CLI tool, you can either compile it from source or install it via a package manager (if available).

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/genproject.git

# Navigate to the project directory
cd genproject

# Build the project using Cargo
cargo build --release

# The binary will be located in target/release/genproject
```

---

## Usage

The CLI supports several flags for creating projects with different configurations. You can use it in two modes:

1. **Non-Interactive Mode**: Specify options through flags and generate the project immediately.
2. **Interactive Mode**: Use the `--interactive` flag for a guided, step-by-step project generation.

### General Syntax

```bash
genproject [OPTIONS]
```

### Flags

| Flag                          | Description                                                                                              | Example                                      |
|-------------------------------|----------------------------------------------------------------------------------------------------------|----------------------------------------------|
| `-n, --project-name <NAME>`    | Set the name of the project. Default: `new_project`.                                                      | `--project-name my_project`                 |
| `-t, --project-type <TYPE>`    | Specify the type of project to generate. Options include `nodejs`, `nestjs`, `rust`.                    | `--project-type nodejs`                     |
| `--output-path <PATH>`         | Set the directory where the project should be created. Default is the current directory (`.`).           | `--output-path /path/to/projects`           |
| `--linter <LINTER>`            | Include a linter in the project. Available linters: `eslint,`, `biome`.                          | `--linter EslintPrettier`                   |
| `-i, --interactive`            | Enable interactive mode for guided setup.                                                               | `--interactive`                             |
| `-h, --help`                   | Display help information about the CLI.                                                                  | `genproject -h`                             |
| `-V, --version`                | Display the version of the CLI.                                                                          | `genproject -V`                             |

---

## Examples

### Example 1: Create a Node.js Project

Generate a Node.js project with the name `my_node_project` in the current directory:

```bash
genproject --project-name my_node_project --project-type nodejs
```

This will create a project named `my_node_project` using the default settings for a Node.js project (Typescript + Eslint)

### Example 2: Create a Rust Project with a Linter

Generate a Rust project with the name `my_rust_project`, including the `Biome` linter, and specify the output directory as `/projects/rust`:

```bash
genproject --project-name my_rust_project --project-type rust --linter biome --output-path /projects/rust
```

This will create a Rust project with `biome` configured as the linter in the specified directory.

### Example 3: Interactive Mode

If you prefer a guided setup, you can use the interactive mode. This will prompt you for the project name, type, and additional settings:

```bash
genproject --interactive
```

You will be prompted with a series of questions like:

1. Enter the project name (default: `new_project`)
2. Select the project type (e.g., `nodejs`, `nestjs`, `rust`, `deno`)
3. Select a linter (optional)

---

## Detailed Description of Flags

### `--project-name`
Defines the name of the project to generate. If not provided, the default value is `new_project`.

### `--output-path`
Specifies the directory where the project should be generated. By default, it creates the project in the current directory (`.`).

### `--project-type`
This flag defines the type of project to create. Supported types include:
- `nodejs`: Node.js project template
- `nestjs`: NestJS project template
- `rust`: Rust project template

### `--linter`
This flag allows you to specify which linter to include in the project. Supported linters are:
- `eslint`: A configuration combining ESLint and Prettier for JavaScript/TypeScript projects.
- `biome`: A newer, faster linter with support for JavaScript and TypeScript projects.

If not specified, no linter is added to the project.

### `--interactive`
If this flag is passed, the tool will enter interactive mode, where you will be prompted for the necessary information to create the project. This mode is useful if you are unsure about the project configuration or want step-by-step guidance.

### `--help`
Displays a help message describing the available flags and how to use them.

### `--version`
Displays the current version of the CLI tool.

---

## Contributing

If you would like to contribute to the development of this project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with clear messages.
4. Push your changes to your fork and submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

### Example Interactive Flow

When using `--interactive`, the CLI will guide you through a series of questions, like so:

```bash
$ genproject --interactive
Welcome to the project generator!  

Welcome to the project generator!
✔ Enter the project name · new_project

? Select project type: 
  ◯ nodejs             
  ◯ nestjs
  ◉ rust
  ◯ deno              

? Select linter: 
  ◯ eslint
  ◉ biome

? Enter the output path (default: current directory):  .
  
Project details:  
Project Name: new_project
Selected Project Type: rust
Selected Linter: Biome
Output Path: .

Creating your project...
```