extern crate rusqlite;
extern crate serde_json;
extern crate dirs;

// use rusqlite::{Connection, Result};
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde_json as json;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileEntry {
    pub id: u32,
    // path: &'a Path,
    pub path: String,
    pub cmds: Vec<String>
}

pub fn conf_dir() -> PathBuf {
    dirs::config_dir().expect("Can't find user's configuration directory.")
}

pub fn db_path() -> PathBuf {
    let mut path = conf_dir();
    path.push("super_pipe");
    path.push("files");
    path.set_extension("db");
    path
}

pub fn db_conn() -> rusqlite::Connection {
    let dbf: String = String::from(db_path().into_os_string().into_string().expect("Unsupported OS type---don't know how to deal with your pathname."));
    Connection::open(dbf).expect("Could not open files.db for some reason.")
}

/// Ensure that there is a database
pub fn init() {
    let conn = db_conn();
    conn.execute(
        "create table if not exists files (
             id integer primary key,
             path text not null unique,
             commands text null
         )",
        NO_PARAMS,
    ).expect("Could not create table in database");
}

pub enum IoDbError {
    Io(std::io::Error),
    Db(rusqlite::Error)
}

impl std::fmt::Display for IoDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IoDbError::Io(ref err) => err.fmt(f),
            IoDbError::Db(ref err) => err.fmt(f)
        }
    }
}

// FIXME: This is not a good implementation for fmt::Debug, but I'll do for now
impl std::fmt::Debug for IoDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IoDbError::Io(ref err) => err.fmt(f),
            IoDbError::Db(ref err) => err.fmt(f)
        }
    }
}

impl std::error::Error for IoDbError {
    fn description(&self) -> &str {
        match *self {
            IoDbError::Io(ref err) => err.description(),
            IoDbError::Db(ref err) => err.description()
        }
    }
}

impl From<std::io::Error> for IoDbError {
    fn from(err: std::io::Error) -> IoDbError {
        IoDbError::Io(err)
    }
}
impl From<rusqlite::Error> for IoDbError {
    fn from(err: rusqlite::Error) -> IoDbError {
        IoDbError::Db(err)
    }
}
impl From<serde_json::Error> for IoDbError {
    fn from(err: serde_json::Error) -> IoDbError {
        IoDbError::Io(std::io::Error::from(err))
    }
}

pub fn add_path<'a>(path: PathBuf, commands: Vec<String>) -> Result<&'a str, IoDbError> {
    // TODO: Add timestamps
    let path = path.to_str().expect("Could not convert path into string");

    let conn = db_conn();
    let cmds = json::to_string(&commands)?;

    conn.execute("INSERT INTO files (path, commands) VALUES (?1, ?2)",
                 &[path, cmds.as_str()])?;
    
    println!("Pipeline for {} inserted", path);
    Ok("Pipeline inserted")
}

pub fn delete_path(id: u32) {
    let conn = db_conn();

    conn.execute("DELETE FROM files WHERE id = ?1", &[id])
        .expect("Could not delete path from database");
}

pub fn fetch_pipeline(id: u32) -> Result<FileEntry, IoDbError> {
    let conn = db_conn();
    let mut stmt = conn.prepare("SELECT id, path, commands FROM files WHERE id = ?1;")
        .expect("Couldn't prepare statement");

    Ok(stmt.query_row(&[id], |row| {
	let cmds: String = row.get(2)?;
	Ok(FileEntry {
	    id: row.get(0)?,
	    path: row.get(1)?,
	    cmds: json::from_str(cmds.as_str()).expect("Malformed JSON in result row: I can't convert the types well enough to recover. ¯\\_(ツ)_/¯")
	})
    })?)
}

pub fn list_paths<'a>() -> Result<Vec<Result<FileEntry, &'a str>>, &'a str> {
    let conn = db_conn();
    let mut stmt = conn.prepare("SELECT id, path, commands FROM files;")
        .expect("Couldn't prepare statement");


    let paths = stmt.query_map(NO_PARAMS, |row| {
	let id: u32 = row.get(0).expect("Couldn't fetch ID");
        let path: String = row.get(1).expect("Couldn't fetch first param");
        let cmds: String = row.get(2).expect("Couldn't fetch first param");
        Ok(FileEntry { id, path, cmds: json::from_str(cmds.as_str()).expect("Couldn't parse commands") })
    }).expect("Couldn't retrieve rows!");

    let mut ret = Vec::new();
    for path in paths {
        match path {
            Ok(fe) => {
                ret.push(Ok(fe))
            },
            Err(_) => return Err("Serious problems here")
        }
    }

    // CLEANUP: I think .collect would be better here

    Ok(ret)
}
