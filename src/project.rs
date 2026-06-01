use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use log::{error, info, log, warn};
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
    let target: Option<&Path> = if let Some(directory) = &directory {
        Some(Path::new(directory))
    } else {
        None
    };

    let current_dir = {
        let path = if let Some(target) = target {
            target
        } else {
            Path::new("./")
        };
        match fs::read_dir(path) {
            Ok(dir) => {
                match dir
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()
                {
                    Ok(res) => res,
                    Err(e) => {
                        error!("Failed to collect current directory's files {}", e);
                        return;
                    }
                }
            }
            Err(e) => {
                error!("Failed to get current directory! {}", e);
                return;
            }
        }
    };

    if current_dir.len() > 0 {
        warn!(
            "Current directory has other items in it, are you sure you want to initalize a project here? (y/n)"
        );
        let mut choice: String = String::new();
        while choice.is_empty() {
            io::stdin()
                .read_line(&mut choice)
                .expect("Unable to read user input.");

            if choice.trim().to_lowercase() == "y" {
            } else if choice.trim().to_lowercase() == "n" {
                info!("Exiting...");
                return;
            } else {
                choice = String::new();
            }
        }
    }

    let project_file = if let Some(target) = target {
        if !target.exists() {
            match std::fs::create_dir(&target) {
                Ok(_) => info!("Creating directory {}", target.to_string_lossy()),
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

pub fn build() {
    todo!();
}
