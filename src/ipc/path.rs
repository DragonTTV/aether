use std::path::PathBuf;

pub fn socket_path() -> PathBuf {
    let mut path = dirs::runtime_dir().expect("No runtime directory");
    path.push("aether");

    std::fs::create_dir_all(&path).unwrap();

    path.push("aether.sock");
    path
}
