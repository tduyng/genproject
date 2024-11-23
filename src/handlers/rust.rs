use crate::templates::TemplateCopier;
use std::path::PathBuf;

pub fn handle_rust(cli: crate::cli::Cli) {
    let template_dir = if cli.project_type == "rust" {
        "templates/rust/default"
    } else {
        "templates/rust/workspace"
    };
    let output_dir = PathBuf::from(&cli.output_path).join(&cli.project_name);
    let copier = TemplateCopier::new(
        template_dir,
        output_dir.to_str().expect("Invalid output path"),
    );

    copier.copy().expect("Failed to generate Rust project");
}
