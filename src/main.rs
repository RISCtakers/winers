use winers::Wine;
use std::env;

fn main() {
    println!("winers debug - meow :3");
    
    let home = env::var("HOME").expect("could not get the HOME variable");

    let prefix_path = format!("{}/.wine", home);
    let prefix = Wine::new(&prefix_path);

    match prefix.init() {
        Ok(_) => println!("Wine prefix initialized successfully at '{}'.", prefix_path),
        Err(e) => println!("Error: {}", e)
    };

    let output = prefix.cmd()
        .arg("notepad")
        .output()
        .expect("how the hell did notepad fail to run");

    println!("status: {}", output.status);

    match prefix.reg_add(r"HKEY_CURRENT_USER\WINERS", "DidItWork", "REG_DWORD", "1") {
        Ok(_) => println!("Successfully created the registry key!"),
        Err(e) => println!("Error: {}", e)
    };

    prefix.cmd()
        .arg("regedit")
        .output()
        .expect("regedit startup epic failure");

    match prefix.reg_delete(r"HKEY_CURRENT_USER\WINERS", "DidItWork") {
        Ok(_) => println!("Successfully deleted the registry key!"),
        Err(e) => println!("Error: {}", e)
    };

    prefix.cmd()
        .arg("regedit")
        .output()
        .expect("regedit startup epic failure");

    match prefix.delete() {
        Ok(_) => println!("Wine prefix deleted successfully at '{}'.", prefix_path),
        Err(e) => print!("Error: {}", e.to_string())
    };
}