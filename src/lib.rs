pub mod store;

pub fn add_path(path: std::path::PathBuf, cmds: Vec<String>) {
    println!("Adding path: {:?}, cmds: {:?}", path, cmds);
    store::add_path(path, cmds);
}

pub fn list_paths() {
    println!("Known paths:");
    store::list_paths();
}
