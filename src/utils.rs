use std::{path::Path, process::Command};

pub(crate) fn cmd(prefix_path: &Path) -> Command {
    let mut cmd = Command::new("wine");
        cmd.env("WINEPREFIX", prefix_path);
    
    cmd
}

pub(crate) fn initialize_wine_prefix(prefix_path: &Path) -> Result<(), String> {
    let output = Command::new("wine")
        .arg("wineboot")
        .arg("--init")
        .env("WINEPREFIX", prefix_path)
        .output()
        .map_err(|e| format!("Failed to execute wine: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Wine prefix initialization failed: {}", stderr));
    }

    Ok(())
}