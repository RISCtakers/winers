use crate::Wine;

pub(crate) fn initialize_wine_prefix(prefix: &Wine) -> Result<(), String> {
    let output = prefix.cmd()
        .arg("wineboot")
        .arg("--init")
        .output()
        .map_err(|e| format!("Failed to execute wine: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Wine prefix initialization failed: {}", stderr));
    }

    Ok(())
}