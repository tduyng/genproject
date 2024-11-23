use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

pub struct TemplateCopier {
    source_dirs: Vec<(PathBuf, Vec<String>)>,
    destination: PathBuf,
}

impl TemplateCopier {
    pub fn new(destination: &str) -> Self {
        Self {
            source_dirs: Vec::new(),
            destination: PathBuf::from(destination),
        }
    }

    pub fn add_source_with_exclusions(&mut self, source: &str, exclusions: Vec<&str>) -> &mut Self {
        self.source_dirs.push((
            PathBuf::from(source),
            exclusions.iter().map(|s| s.to_string()).collect(),
        ));
        self
    }

    pub fn copy(&self) -> std::io::Result<()> {
        create_dir_all(&self.destination)?;
        for (source_dir, exclusions) in &self.source_dirs {
            if source_dir.exists() {
                Self::copy_recursive(source_dir, &self.destination, exclusions)?;
            }
        }
        Ok(())
    }

    fn copy_recursive(src: &Path, dest: &Path, exclusions: &[String]) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path
                .strip_prefix(src)
                .expect("Failed to compute relative path");
            let dest_path = dest.join(relative_path);

            if exclusions.iter().any(|ex| path.ends_with(ex)) {
                continue; // Skip excluded files
            }

            if path.is_dir() {
                create_dir_all(&dest_path)?; // Preserve subdirectories
                Self::copy_recursive(&path, &dest_path, exclusions)?;
            } else {
                create_dir_all(dest_path.parent().unwrap())?; // Ensure parent directories exist
                fs::copy(&path, &dest_path)?;
            }
        }
        Ok(())
    }
}
