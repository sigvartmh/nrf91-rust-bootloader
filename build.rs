use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", manifest_dir);
    println!("cargo:rustc-link-search={}","/home/siho/Nordic/nrf9160-rust-bootloader/src");
    println!("cargo:rustc-link-search={}","/home/siho/Nordic/nrf9160-rust-bootloader");
    println!("cargo:rustc-link-search={}/libraries/usr/lib/arm-linux-gnueabihf", manifest_dir);
    println!("cargo:rustc-link-lib=static=nrf_cc310_bl_0.9.12");
}
