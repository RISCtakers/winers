use std::env;
use winers::{get_latest_dxvk, install_dxvk, Wine};

#[tokio::main]
async fn main() {
    println!("winers debug - meow :3");

    let home = env::var("HOME").expect("could not get the HOME variable");

    let prefix_path = format!("{}/.wine", home);
    let prefix = Wine::new(&prefix_path, None);

    match prefix.init() {
        Ok(_) => println!("Wine prefix initialized successfully at '{}'.", prefix_path),
        Err(e) => println!("Error: {}", e),
    };

    let output = prefix
        .cmd()
        .arg("notepad")
        .output()
        .expect("how the hell did notepad fail to run");

    println!("status: {}", output.status);

    match prefix.reg_add(r"HKEY_CURRENT_USER\WINERS", "DidItWork", "REG_DWORD", "1") {
        Ok(_) => println!("Successfully created the registry key!"),
        Err(e) => println!("Error: {}", e),
    };

    prefix
        .cmd()
        .arg("regedit")
        .output()
        .expect("regedit startup epic failure");

    match prefix.reg_delete(r"HKEY_CURRENT_USER\WINERS", "DidItWork") {
        Ok(_) => println!("Successfully deleted the registry key!"),
        Err(e) => println!("Error: {}", e),
    };

    prefix
        .cmd()
        .arg("regedit")
        .output()
        .expect("regedit startup epic failure");

    let latest_dxvk = get_latest_dxvk()
        .await
        .expect("epic dxvk version fetch fail");
    match install_dxvk(&prefix, latest_dxvk.as_str()).await {
        Ok(_) => println!("Successfully installed dxvk."),
        Err(e) => eprintln!("Error installing dxvk: {e}"),
    }

    match prefix.delete() {
        Ok(_) => println!("Wine prefix deleted successfully at '{}'.", prefix_path),
        Err(e) => print!("Error: {}", e.to_string()),
    };
}
