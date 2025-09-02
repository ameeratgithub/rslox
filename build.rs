// build.rs
fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-arg-bin=rslox=/STACK:16777216"); // 16 MB
}
