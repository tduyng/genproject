use crate::{
    cli::{Linter, SubType},
    templates::TemplateCopier,
};
use std::path::PathBuf;

pub fn handle_nodejs(cli: crate::cli::Cli) {
    let subtype_dir = match (cli.subtype, cli.linter) {
        (Some(SubType::Js), Some(Linter::Eslint)) => "templates/nodejs/js/eslint",
        (Some(SubType::Js), Some(Linter::Biome)) => "templates/nodejs/js/biome",
        (Some(SubType::Ts), Some(Linter::Eslint)) => "templates/nodejs/ts/eslint",
        (Some(SubType::Ts), Some(Linter::Biome)) => "templates/nodejs/ts/biome",
        (Some(SubType::Js), None) => "templates/nodejs/js",
        (Some(SubType::Ts), None) => "templates/nodejs/ts",
        _ => "templates/nodejs/ts/eslint",
    };
    let output_dir = PathBuf::from(&cli.output_path).join(&cli.project_name);
    let copier = TemplateCopier::new(
        subtype_dir,
        output_dir.to_str().expect("Invalid output path"),
    );

    copier.copy().expect("Failed to generate Nestjs project");
}
