// build.rs
fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-arg-bin=rslox=/STACK:16777216"); // 16 MB

    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-arg-bin=rslox=-Wl,--stack,16777216"); // 16 MB
}
