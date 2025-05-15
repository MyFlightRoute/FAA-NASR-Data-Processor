use std::fs;
use std::io::Write;

fn main() {
    let path = "build/build_number.txt";

    // Read the current build number or default to 0
    let current = fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);

    let new_build = current + 1;

    // Save new build number back to file
    fs::write(path, new_build.to_string()).expect("Failed to write build number");

    // Write a Rust file with the constant
    let mut f = fs::File::create("src/build.rs").expect("Failed to create src/build.rs");
    writeln!(f, "pub const BUILD_NUMBER: &str = \"{}\";", new_build).unwrap();
}
