use crate::templates::TemplateCopier;
use std::path::PathBuf;

pub fn handle_deno(cli: crate::cli::Cli) {
    let template_dir = "templates/rust/default";
    let output_dir = PathBuf::from(&cli.output_path).join(&cli.project_name);
    let copier = TemplateCopier::new(
        template_dir,
        output_dir.to_str().expect("Invalid output path"),
    );

    copier.copy().expect("Failed to generate Rust project");
}
