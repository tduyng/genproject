use clap::{Parser, ValueEnum};
use colored::Colorize;
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
        println!(
            "{}",
            "\nWelcome to the project generator!".bold().bright_green()
        );

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
        use dialoguer::{theme::ColorfulTheme, Select};
        let theme = ColorfulTheme::default();

        let project_name: String = Input::with_theme(&theme)
            .with_prompt("Enter the project name")
            .default("new_project".to_string())
            .interact_text()
            .expect("Failed to read input");

        let project_types = vec!["nodejs", "nestjs", "rust", "deno"];
        let selected_index = Select::with_theme(&theme)
            .with_prompt("Select project type")
            .items(&project_types)
            .interact()
            .unwrap();
        let project_type = project_types[selected_index].to_string();

        let linters = vec!["eslint", "biome"];
        let linter_index = Select::with_theme(&theme)
            .with_prompt("Select linter")
            .items(&linters)
            .default(0)
            .interact()
            .expect("Failed to select linter");
        let linter = match linter_index {
            0 => Some(Linter::Eslint),
            1 => Some(Linter::Biome),
            _ => Some(Linter::Eslint),
        };

        let output_path: String = Input::with_theme(&theme)
            .with_prompt("Enter the output path (default: current directory)")
            .default(".".to_string())
            .interact_text()
            .unwrap();

        println!("{}", "\nProject details:".bold().blue());

        println!("  Project Name: {}", project_name.green());
        println!("  Project Type: {}", project_type.yellow());
        println!(
            "  Selected Linter: {}",
            linter
                .clone()
                .map_or("None", |l| match l {
                    Linter::Eslint => "eslint",
                    Linter::Biome => "biome",
                })
                .magenta()
        );
        println!("  Output Path: {}", output_path.cyan());

        println!("{}", "\nCreating your project...");

        Cli {
            project_name,
            output_path,
            project_type,
            linter,
            interactive: true,
        }
    }
}
