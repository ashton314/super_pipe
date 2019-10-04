pub mod store;

pub fn add_path(path: std::path::PathBuf, cmds: Vec<String>) {
    println!("Adding path: {:?}, cmds: {:?}", path, cmds);
    match store::add_path(path, cmds) {
        Ok(_) => println!("Done."),
        Err(e) => {
            println!("There was a problem:");
            match e {
                store::IoDbError::Db(rusqlite::Error::SqliteFailure(_, Some(description))) => println!("Error from SQL: {}", description),
                _ => println!("{:?}", e)
            }
        }
    };
}

pub fn list_paths() {
    println!("Known paths:");
    store::list_paths();
}

pub fn delete_pipe(id: u32) {
    println!("Deleting path {:?}", id);
    store::delete_path(id);
}
