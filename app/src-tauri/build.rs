fn main() {
    println!(
        "cargo:rustc-env=JOMAI_BUILD_TIMESTAMP={}",
        chrono::Utc::now().to_rfc3339()
    );
    tauri_build::build()
}
