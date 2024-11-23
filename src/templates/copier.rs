use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

pub struct TemplateCopier {
    source: PathBuf,
    destination: PathBuf,
}

impl TemplateCopier {
    pub fn new(source: &str, destination: &str) -> Self {
        Self {
            source: PathBuf::from(source),
            destination: PathBuf::from(destination),
        }
    }

    pub fn copy(&self) -> std::io::Result<()> {
        create_dir_all(&self.destination)?;
        TemplateCopier::copy_recursive(&self.source, &self.destination)?;
        Ok(())
    }

    fn copy_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dest.join(entry.file_name());
            if path.is_dir() {
                create_dir_all(&dest_path)?;
                TemplateCopier::copy_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }
        Ok(())
    }
}
