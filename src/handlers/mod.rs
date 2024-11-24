use std::path::Path;

use crate::cli::Cli;
use crate::templates::{copier::TemplateCopier, package::Package};

pub fn handle_project(cli: &Cli) {
    let output_dir = format!("{}/{}", cli.output_path, cli.project_name);
    std::fs::create_dir_all(&output_dir).expect("Failed to create project directory");

    let mut copier = TemplateCopier::new(&output_dir);

    // Collect all source paths and their exclusions
    let mut sources = vec![(
        format!("templates/{}/core", cli.project_type),
        vec!["package.json"],
    )];

    if let Some(linter) = &cli.linter {
        sources.push((
            format!("templates/{}/{linter}", cli.project_type),
            vec!["package.json"],
        ));
    }

    if let Some(test_framework) = &cli.test_framework {
        sources.push((
            format!("templates/{}/{test_framework}", cli.project_type),
            vec!["package.json"],
        ));
    }

    // Add sources to the copier
    for (path, exclusions) in sources {
        copier.add_source_with_exclusions(&path, exclusions);
    }

    copier.copy().expect("Failed to copy templates");

    // Merge all package.json files dynamically
    let package_paths = vec![
        Some(format!("templates/{}/core/package.json", cli.project_type)),
        cli.linter
            .as_ref()
            .map(|linter| format!("templates/{}/{}/package.json", cli.project_type, linter)),
        cli.test_framework
            .as_ref()
            .map(|test| format!("templates/{}/{}/package.json", cli.project_type, test)),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let mut merged_package =
        Package::from_file(&format!("templates/{}/core/package.json", cli.project_type));
    for path in package_paths.into_iter() {
        if Path::new(&path).exists() {
            let child_package = Package::from_file(&path);
            merged_package.merge_with(&child_package);
        }
    }

    let output_package_path = format!("{}/package.json", output_dir);
    merged_package
        .save_to_file(&output_package_path)
        .expect("Failed to write merged package.json");
}
