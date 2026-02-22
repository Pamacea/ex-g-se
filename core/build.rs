use std::env;

fn main() {
    // Platform-specific build instructions
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "linux" => {
            println!("cargo:warning=Building for Linux - ensure libx11-dev, libxtst-dev, and libxrandr-dev are installed");
        }
        "macos" => {
            println!(
                "cargo:warning=Building for macOS - ensure Accessibility permissions are granted"
            );
        }
        "windows" => {
            println!("cargo:warning=Building for Windows - no additional dependencies required");
        }
        _ => {
            println!(
                "cargo:warning=Building for unsupported target: {}",
                target_os
            );
        }
    }
}
