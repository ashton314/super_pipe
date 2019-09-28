extern crate rusqlite;
extern crate serde_json;

// use rusqlite::{Connection, Result};
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde_json as json;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    // path: &'a Path,
    path: String,
    cmds: Vec<String>
}

/// Ensure that there is a database
pub fn init() {
    let dbf: String = String::from("/Users/ashton/.sup/files.db");
    let conn = Connection::open(dbf).expect("Could not open files.db for some reason.");

    conn.execute(
        "create table if not exists files (
             id integer primary key,
             path text not null unique,
             commands text null
         )",
        NO_PARAMS,
    ).expect("Could not create table in database");
}

pub fn add_path(path: PathBuf, commands: Vec<String>) {
    // TODO: Add timestamps
    let path = path.to_str().expect("Could not convert path into string");

    let dbf: String = String::from("/Users/ashton/.sup/files.db");
    let conn = Connection::open(dbf)
        .expect("Could not open files.db for some reason.");
    let cmds = json::to_string(&commands)
        .expect("Unable to serialize command list");

    conn.execute("INSERT INTO files (path, commands) VALUES (?1, ?2)",
                 &[path, cmds.as_str()])
        .expect("Unable to insert into database");
    
    println!("Pipeline for {} inserted", path);
}

pub fn delete_path(path: PathBuf) {
    let path = path.to_str().expect("Could not convert path into string");

    let dbf: String = String::from("/Users/ashton/.sup/files.db");
    let conn = Connection::open(&dbf)
        .expect("Could not open files.db for some reason.");

    conn.execute("DELETE FROM files WHERE path = ?1", &[path])
        .expect("Could not delete path from database");
}

pub fn list_paths() {
    let dbf: String = String::from("/Users/ashton/.sup/files.db");
    let conn = Connection::open(&dbf)
        .expect("Could not open files.db for some reason.");

    let mut stmt = conn.prepare("SELECT path, commands FROM files;")
        .expect("Couldn't prepare statement");

    let paths = stmt.query_map(NO_PARAMS, |row| {
        let path: String = row.get(0).expect("Couldn't fetch first param");
        let cmds: String = row.get(1).expect("Couldn't fetch first param");
        Ok(FileEntry { path, cmds: json::from_str(cmds.as_str()).expect("Couldn't parse commands") })
    })
        .expect("Could not list rows");

    for path in paths {
        match path {
            Ok(FileEntry {path, cmds}) => {
                println!("{}: {:?}", path, cmds);
            },
            Err(_) => panic!("Got something that wasn't OK")
        }

    }
}
