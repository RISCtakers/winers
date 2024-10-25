use crate::{utils, Wine};
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::Client;

const OWNER: &str = "doitsujin";
const REPO: &str = "dxvk";

pub async fn get_latest_dxvk() -> Result<String, reqwest::Error> {
    utils::get_latest_release(&OWNER, &REPO).await
}

pub async fn install_dxvk(prefix: &Wine, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://github.com/{}/{}/releases/download/{}/dxvk-{}.tar.gz",
        OWNER, REPO, version, &version[1..]
    );

    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    let gz = GzDecoder::new(bytes.as_ref());
    let mut archive = Archive::new(gz);

    let x64_dir = &prefix.prefix_path.join("drive_c/windows/system32");
    let x32_dir = &prefix.prefix_path.join("drive_c/windows/syswow64");

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if let Some(path_str) = path.to_str() {
            if path_str.to_lowercase().ends_with(".dll") {
                let target_dir = if path_str.contains("x64") {
                    x64_dir
                } else if path_str.contains("x32") {
                    x32_dir
                } else {
                    continue;
                };

                let file_name = path.file_name().unwrap_or_default();
                let target_path = target_dir.join(file_name);
                entry.unpack(&target_path)?;
                println!("Extracted: {}", target_path.display());
            }
        }
    }

    Ok(())
}