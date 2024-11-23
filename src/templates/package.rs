use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize, Default)]
pub struct Package {
    #[serde(flatten)]
    other_fields: HashMap<String, serde_json::Value>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

impl Package {
    pub fn from_file(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).expect("Failed to read package.json");
        serde_json::from_str(&contents).expect("Failed to parse package.json")
    }

    pub fn merge_with(&mut self, other: &Package) {
        self.merge_dependencies("dependencies", &other.dependencies);
        self.merge_dependencies("devDependencies", &other.dev_dependencies);
    }

    fn merge_dependencies(&mut self, field: &str, other_deps: &Option<HashMap<String, String>>) {
        let self_deps = match field {
            "dependencies" => self.dependencies.get_or_insert_with(HashMap::new),
            "devDependencies" => self.dev_dependencies.get_or_insert_with(HashMap::new),
            _ => panic!("Unknown field: {}", field),
        };

        if let Some(other_map) = other_deps {
            for (key, value) in other_map {
                self_deps
                    .entry(key.clone())
                    .or_insert_with(|| value.clone());
            }
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }
}
