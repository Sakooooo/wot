use std::{fs, path::Path};

use log::{error, info, log};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::from("My cool website"),
            version: String::from(env!("CARGO_PKG_VERSION")),
        }
    }
}

pub fn init(directory: Option<String>) {
    let project_file = if let Some(directory) = &directory {
        let target = Path::new(directory);
        if !target.exists() {
            match std::fs::create_dir(&target) {
                Ok(_) => info!("Creating directory {}", directory),
                Err(e) => {
                    error!("Failed to create directory! {}", e);
                    return;
                }
            };
        }
        target.join("wot.toml")
    } else {
        Path::new("wot.toml").to_path_buf()
    };

    if project_file.exists() {
        error!("A project already exists here!");
        info!("hint: remove wot.toml");
        return;
    };

    let default = Project::default();
    let contents = match toml::to_string_pretty(&default) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Failed to turn default values into wot.toml! {}", e);
            return;
        }
    };

    match std::fs::write(project_file, contents) {
        Ok(_) => info!("Successfully initalized project!"),
        Err(e) => error!("Failed to write project file {}", e),
    };
}
