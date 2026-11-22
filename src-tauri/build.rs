fn main() {
    // Ensure bundled Windows CLI staging directory exists so empty-dir resource
    // walking succeeds during `cargo test` even when CLI binary is not staged yet.
    let _ = std::fs::create_dir_all("resources/bin");
    tauri_build::build()
}
