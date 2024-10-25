use crate::utils;

const OWNER: &str = "doitsujin";
const REPO: &str = "dxvk";

pub async fn get_latest_dxvk() {
    match utils::get_latest_release(OWNER, REPO).await {
        Ok(tag) => println!("Latest release tag: {}", tag),
        Err(e) => eprintln!("Error fetching release: {}", e),
    }
}