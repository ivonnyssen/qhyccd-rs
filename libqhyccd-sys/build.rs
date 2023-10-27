use std::{env, path::Path};

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();
    // Specify `LIBQHYCCD_NO_VENDOR` to force to use system libqhyccd.
    // Due to the additive nature of Cargo features, if some crate in the
    // dependency graph activates `vendored` feature, there is no way to revert
    // it back. This env var serves as a workaround for this purpose.
    println!("cargo:rerun-if-env-changed=LIBQHYCCD_NO_VENDOR");
    let forced_no_vendor = env::var_os("LIBQHYCCD_NO_VENDOR").map_or(false, |s| s != "0");
    match vendored && !forced_no_vendor {
        true => {
            let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            println!(
                "cargo:rustc-link-search=native={}",
                Path::new(&dir).join("qhyccd-sdk").join(arch).display()
            );
            println!("cargo:rustc-cfg=libqhyccd_vendored");
        }
        false => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
    }
    println!("cargo:rustc-link-lib=static=qhyccd");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
