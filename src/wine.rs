use std::{fs, io, path::PathBuf, process::Command};

pub struct Wine {
    pub prefix_path: PathBuf,
    pub wine_root: Option<PathBuf>
}

impl Wine {
    pub fn new(prefix_path: &str, wine_root: Option<String>) -> Self {
        Wine {
            prefix_path: PathBuf::from(prefix_path),
            wine_root: wine_root.map(PathBuf::from)
        }
    }

    pub fn init(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-i")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix initialization failed: {}", stderr));
        }

        Ok(())
    }

    pub fn update(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-u")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix updating failed: {}", stderr));
        }

        Ok(())
    }

    pub fn kill(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-k")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix killing failed: {}", stderr));
        }

        Ok(())
    }

    pub fn delete(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.prefix_path)
    }

    pub fn cmd(&self) -> Command {
        let wine_binary = self.wine_root
            .as_ref()
            .map(|root| root.join("bin").join("wine64"))
            .unwrap_or_else(|| PathBuf::from("wine64"));

        let mut cmd = Command::new(wine_binary);
        cmd.env("WINEPREFIX", &self.prefix_path);

        cmd
    }

    pub fn reg_add(&self, key: &str, value: &str, reg_type: &str, data: &str) -> Result<(), String> {
        let output = self.cmd()
            .args(["reg", "add", key, "/v", value, "/t", reg_type, "/d", data, "/f"])
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to add registry key: {}", stderr));
        }

        Ok(())
    }

    pub fn reg_delete(&self, key: &str, value: &str) -> Result<(), String> {
        let output = self.cmd()
            .args(["reg", "delete", key, "/v", value, "/f"])
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to delete registry key: {}", stderr));
        }

        Ok(())
    }
}