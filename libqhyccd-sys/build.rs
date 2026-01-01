fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "macos" => {
            // Check for SDK in workspace first (CI environment)
            if let Ok(workspace) = std::env::var("GITHUB_WORKSPACE") {
                let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
                let sdk_path = if arch == "aarch64" {
                    format!("{}/sdk_mac_arm_25.09.29", workspace)
                } else {
                    format!("{}/sdk_macMix_25.09.29", workspace)
                };
                println!("cargo:rustc-link-search=native={}", sdk_path);
            } else {
                // Fallback to system installation
                println!("cargo:rustc-link-search=native=/usr/local/lib");
            }
            println!("cargo:rustc-link-lib=static=qhyccd");
            // macOS SDK likely includes USB support or uses system frameworks
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
        "windows" => {
            if let Ok(workspace) = std::env::var("GITHUB_WORKSPACE") {
                println!("cargo:rustc-link-search=native={}/sdk_WinMix_25.09.29", workspace);
            }
            println!("cargo:rustc-link-lib=static=qhyccd");
            // Windows SDK likely includes all dependencies
        }
        _ => {
            // Linux and other Unix-like systems
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=static=qhyccd");
            println!("cargo:rustc-link-lib=dylib=usb-1.0");
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}
