use std::process::Command;

fn main() {
    // Try to find CBC library path automatically

    // On macOS with Homebrew, try to get the path dynamically
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("brew").args(["--prefix", "cbc"]).output() {
            if output.status.success() {
                let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("cargo:rustc-link-search=native={}/lib", prefix);
                return;
            }
        }
    }

    // On Linux, try pkg-config
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("pkg-config").args(["--libs-only-L", "cbc"]).output() {
            if output.status.success() {
                let libs = String::from_utf8_lossy(&output.stdout);
                for lib in libs.split_whitespace() {
                    if let Some(path) = lib.strip_prefix("-L") {
                        println!("cargo:rustc-link-search=native={}", path);
                    }
                }
                return;
            }
        }
    }

    // If we get here, we couldn't find CBC automatically
    // Print a helpful error message
    eprintln!("Warning: Could not automatically locate CBC library.");
    eprintln!("Please ensure CBC is installed:");
    eprintln!("  - macOS: brew install cbc");
    eprintln!("  - Linux: sudo apt-get install coinor-libcbc-dev");
}