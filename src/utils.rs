use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

pub async fn get_latest_release(owner: &str, repo: &str) -> Result<String, Error> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);
    
    let client = reqwest::Client::new();
    let release: Release = client
        .get(&url)
        .header("User-Agent", "WineRS")
        .send()
        .await?
        .json()
        .await?;

    Ok(release.tag_name)
}