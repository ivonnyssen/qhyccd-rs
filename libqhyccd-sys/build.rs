use std::{env, path::PathBuf};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "macos" => {
            // Check for SDK in workspace first (CI environment)
            if let Ok(workspace) = env::var("GITHUB_WORKSPACE") {
                let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
                let sdk_path = if arch == "aarch64" {
                    format!("{}/sdk_mac_arm_25.09.29/usr/local/lib", workspace)
                } else {
                    format!("{}/sdk_macMix_25.09.29/usr/local/lib", workspace)
                };
                println!("cargo:warning=Looking for macOS SDK in GITHUB_WORKSPACE: {}", workspace);
                println!("cargo:warning=Architecture: {}, SDK path: {}", arch, sdk_path);
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
            let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            let arch_dir = match arch.as_str() {
                "x86_64" => "x64",
                "x86" | "i686" => "x86",
                other => {
                    println!("cargo:warning=Unknown Windows arch '{}', defaulting to x64", other);
                    "x64"
                }
            };

            if let Ok(workspace) = env::var("GITHUB_WORKSPACE") {
                let ws_sdk = PathBuf::from(&workspace).join("pkg_win");
                println!("cargo:rustc-link-search=native={}", ws_sdk.display());
                println!("cargo:rustc-link-search=native={}", ws_sdk.join(arch_dir).display());
            }

            let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            let sdk_dir = manifest_dir
                .join("qhyccd-sdk")
                .join("pkg_win")
                .join(arch_dir);
            println!("cargo:rustc-link-search=native={}", sdk_dir.display());
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
