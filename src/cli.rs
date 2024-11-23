use clap::{Parser, ValueEnum};
use dialoguer::Input;

/// CLI for generating starter project templates
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Name of the project
    #[arg(short = 'n', long, default_value = "new_project")]
    pub project_name: String,

    /// Path to create the project (default: current directory)
    #[arg(short, long, default_value = ".")]
    pub output_path: String,

    /// Type of project to generate
    #[arg(short = 't', long, value_enum, default_value = "nodejs")]
    pub project_type: String,

    /// Linters to include
    #[arg(short, long, value_enum)]
    pub linter: Option<Linter>,

    /// Enable interactive mode
    #[arg(short, long)]
    pub interactive: bool,
}

#[derive(ValueEnum, Clone)]
pub enum SubType {
    Js,
    Ts,
}

#[derive(ValueEnum, Clone)]
pub enum Linter {
    Eslint,
    Biome,
}

impl Cli {
    /// Parse flags or fall back to interactive mode
    pub fn parse_or_interactive() -> Self {
        println!("Welcome to the project generator!");
        let args = Cli::try_parse();

        match args {
            Ok(cli) => {
                if cli.interactive {
                    InteractiveCli::run()
                } else {
                    cli
                }
            }

            Err(e)
                if e.kind() == clap::error::ErrorKind::DisplayHelp
                    || e.kind() == clap::error::ErrorKind::DisplayVersion =>
            {
                e.print().expect("Failed to display help/version");
                std::process::exit(0);
            }

            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}

/// Interactive CLI prompts
pub struct InteractiveCli;

impl InteractiveCli {
    pub fn run() -> Cli {
        use dialoguer::Select;

        let project_name: String = Input::new()
            .with_prompt("Enter the project name (default: new_project)")
            .default("new_project".to_string())
            .interact_text()
            .expect("Failed to read input");

        let project_type = Select::new()
            .with_prompt("Select project type")
            .items(&["nodejs", "nestjs", "rust", "deno"])
            .interact()
            .unwrap();

        let linter = match project_type {
            0 | 1 => Some(
                Select::new()
                    .with_prompt("Select linter")
                    .items(&["eslint", "biome"])
                    .interact()
                    .unwrap(),
            ),
            _ => Some(0),
        };

        let output_path: String = Input::new()
            .with_prompt("Enter the output path (default: current directory)")
            .default(".".to_string())
            .interact_text()
            .unwrap();

        println!("\nProject details:");
        println!("  Project Name: {}", project_name);
        println!("  Project Type: {}", project_type);
        println!(
            "   Selected Linter: {}",
            match linter {
                Some(0) => "eslint",
                Some(1) => "biome",
                _ => "eslint",
            }
        );
        println!("  Output Path: {}", output_path);

        // Confirm project creation
        println!("\nCreating your project...");

        Cli {
            project_name,
            output_path,
            project_type: match project_type {
                0 => "nodejs".to_string(),
                1 => "nestjs".to_string(),
                2 => "rust".to_string(),
                3 => "deno".to_string(),
                _ => unreachable!(),
            },
            linter: match linter {
                Some(0) => Some(Linter::Eslint),
                Some(1) => Some(Linter::Biome),
                _ => None,
            },
            interactive: true,
        }
    }
}
