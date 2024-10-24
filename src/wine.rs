use std::{fs, io, path::PathBuf, process::Command};
use crate::utils::initialize_wine_prefix;

pub struct Wine {
    prefix_path: PathBuf,
}

impl Wine {
    pub fn new(prefix_path: &str) -> Self {
        Wine {
            prefix_path: PathBuf::from(prefix_path),
        }
    }

    pub fn init(&self) -> Result<(), String> {
        initialize_wine_prefix(&self.prefix_path)
    }

    pub fn delete(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.prefix_path)
    }

    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new("wine");
        cmd.env("WINEPREFIX", &self.prefix_path);
        cmd
    }
}