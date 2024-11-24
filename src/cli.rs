use clap::{Parser, ValueEnum};
use colored::Colorize;
use dialoguer::Input;

/// CLI for generating starter project templates
#[derive(Parser, Clone, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Name of the project
    #[arg(short = 'n', long, default_value = "new_project")]
    pub project_name: String,

    /// Path to create the project
    #[arg(short, long, default_value = ".")]
    pub output_path: String,

    /// Type of project to generate [possible values: nestjs, nodejs, deno]
    #[arg(short = 't', long, value_enum, default_value = "nodejs")]
    pub project_type: String,

    /// Linters to include
    #[arg(short, long, value_enum)]
    pub linter: Option<Linter>,

    /// Test framework to include
    #[arg(long, value_enum)]
    pub test_framework: Option<TestFramework>,

    /// Enable interactive mode
    #[arg(short, long, conflicts_with_all = &["project_name", "output_path", "project_type", "linter", "test_framework"])]
    pub interactive: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SubType {
    Js,
    Ts,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Linter {
    Eslint,
    Biome,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TestFramework {
    Jest,
    MochaSinon,
    NodeSinon,
    Vitest,
}

impl std::fmt::Display for Linter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Linter::Eslint => write!(f, "eslint"),
            Linter::Biome => write!(f, "biome"),
        }
    }
}

impl std::fmt::Display for TestFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestFramework::Jest => write!(f, "jest"),
            TestFramework::MochaSinon => write!(f, "mocha-sinon"),
            TestFramework::NodeSinon => write!(f, "node-sinon"),
            TestFramework::Vitest => write!(f, "vitest"),
        }
    }
}

impl Cli {
    /// Parse flags or fall back to interactive mode
    pub fn parse_or_interactive() -> Self {
        let args = Cli::try_parse();

        match args {
            Ok(cli) => {
                println!(
                    "{}",
                    "\nWelcome to the project generator!\n"
                        .bold()
                        .bright_green()
                );

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

        let project_types = vec!["nodejs", "nestjs", "deno"];
        let project_type = project_types[Select::with_theme(&theme)
            .with_prompt("Select project type")
            .items(&project_types)
            .default(0)
            .interact()
            .unwrap()]
        .to_string();

        let linter = Select::with_theme(&theme)
            .with_prompt("Select linter")
            .items(&["none", "eslint", "biome"])
            .default(1)
            .interact()
            .ok()
            .and_then(|i| match i {
                1 => Some(Linter::Eslint),
                2 => Some(Linter::Biome),
                _ => None,
            });

        let test_framework = Select::with_theme(&theme)
            .with_prompt("Select test framework")
            .items(&["none", "jest", "vitest", "mocha-sinon", "node-sinon"])
            .default(0)
            .interact()
            .ok()
            .and_then(|i| match i {
                1 => Some(TestFramework::Jest),
                2 => Some(TestFramework::Vitest),
                3 => Some(TestFramework::MochaSinon),
                4 => Some(TestFramework::NodeSinon),
                _ => None,
            });

        let output_path: String = Input::with_theme(&theme)
            .with_prompt("Enter the output path (default: current directory)")
            .default(".".to_string())
            .interact_text()
            .unwrap();

        Cli {
            project_name,
            output_path,
            project_type,
            linter,
            test_framework,
            interactive: true,
        }
    }
}

pub fn print_project_info(cli: &Cli) {
    println!("{}", "\nProject details:".bold().blue());

    println!("  Project name: {}", cli.project_name.green());
    println!("  Project type: {}", cli.project_type.yellow());
    if let Some(ref linter) = cli.linter {
        println!("  Linter: {}", linter.to_string().cyan());
    }
    if let Some(ref test_framework) = cli.test_framework {
        println!("  Test framework: {}", test_framework.to_string().cyan());
    }
    println!("  Output path: {}", cli.output_path.cyan());
    println!("\nCreating your project...");
}
