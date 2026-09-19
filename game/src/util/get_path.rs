use std::path::PathBuf;

pub fn get_root_path_buf() -> PathBuf {
    // 1. Get the directory of the closest Cargo.toml
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // 2. Build a reliable path to your asset file
    PathBuf::from(manifest_dir)
}
pub fn get_path(path: &str) -> String {
    let root_path_buf = get_root_path_buf();
    let path_buf = root_path_buf.join(path.trim());
    String::from(path_buf.to_str().unwrap())
}
