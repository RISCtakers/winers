use crate::Wine;
use crate::utils;

const OWNER: &str = "doitsujin";
const REPO: &str = "dxvk";

pub async fn install_dxvk(prefix: &Wine) {
    match utils::get_latest_release(OWNER, REPO).await {
        Ok(tag) => println!("Latest release tag: {}", tag),
        Err(e) => eprintln!("Error fetching release: {}", e),
    }
}