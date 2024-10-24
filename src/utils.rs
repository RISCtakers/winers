use crate::Wine;

pub(crate) fn initialize_wine_prefix(prefix: &Wine) -> Result<(), String> {
    let output = prefix.cmd()
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

pub(crate) fn update_wine_prefix(prefix: &Wine) -> Result<(), String> {
    let output = prefix.cmd()
        .arg("wineboot")
        .arg("-i")
        .output()
        .map_err(|e| format!("Failed to execute wine: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Wine prefix killing failed: {}", stderr));
    }

    Ok(())
}

pub(crate) fn kill_wine_prefix(prefix: &Wine) -> Result<(), String> {
    let output = prefix.cmd()
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