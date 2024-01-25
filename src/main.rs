use std::fs;

fn main() {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    fs::create_dir_all("./data").expect("Failed to create");
    
    println!("FAA NASR Data Processor v{}", VERSION.unwrap_or("Unknown"));
}
