use std::{fs, path::Path};

use log::{error, info, log};

struct Project {
    name: String,
    version: u32,
}

pub fn init(directory: Option<String>) {
    let project_file = if let Some(directory) = &directory {
        Path::new(directory).join("wot.toml")
    } else {
        Path::new("wot.toml").to_path_buf()
    };

    if project_file.exists() {
        error!("A project already exists here!");
        info!("hint: remove wot.toml");
        return;
    };
}
