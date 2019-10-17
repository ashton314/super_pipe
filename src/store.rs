extern crate serde;
extern crate toml;
extern crate dirs;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
// use toml::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct FilesStore {
    files: Vec<FileEntry>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub id: u32,
    pub path: String,
    pub pipes: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineRecord {
    pub id: u32,
    pub checksum: String,
    pub source: PathBuf
}

/// Return the root of the directory that we can own
pub fn conf_dir() -> PathBuf {
    let mut path = dirs::config_dir()
        .expect("Can't find user's configuration directory.");
    path.push("super_pipe");
    path
}

pub fn pipes_dir() -> PathBuf {
    let mut path = conf_dir();
    path.push("pipes");
    path
}

/// This returns the path to the file that maps files to pipelines
pub fn pipe_map_path() -> PathBuf {
    let mut path = conf_dir();
    path.push("files");
    path.set_extension("toml");
    path
}

/// Makes sure that there is a file store available
pub fn ensure_has_database() {
    let conf = conf_dir();
    let pipes = pipes_dir();
    let fmp  = pipe_map_path();
    if ! conf.exists() {
        fs::create_dir(&conf)
            .expect(format!("Could not create directory {:?} for some reason.", conf).as_str());
    }
    if ! pipes.exists() {
        fs::create_dir(&pipes)
            .expect(format!("Could not create directory {:?} for some reason.", conf).as_str());
    }
    if ! fmp.exists() {
        fs::File::create(&fmp)
            .expect(format!("Could not create new file map path at {:?} for some eason", fmp).as_str());
    }
}

/// Ensure that there is a database
pub fn init() {
    // FIXME: Add TOML init here
}

pub enum IoDbError {
    Io(std::io::Error),
    Db(toml::de::Error)
}

impl std::fmt::Display for IoDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IoDbError::Io(ref err) => err.fmt(f),
            IoDbError::Db(ref err) => err.fmt(f)
        }
    }
}

// FIXME: This is not a good implementation for fmt::Debug, but it'll do for now
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

impl From<toml::de::Error> for IoDbError {
    fn from(err: toml::de::Error) -> IoDbError {
        IoDbError::Db(err)
    }
}
// impl From<toml::ser::Error> for IoDbError {
//     fn from(err: toml::ser::Error) -> IoDbError {
//         IoDbError::Db(err)
//     }
// }

impl From<std::io::Error> for IoDbError {
    fn from(err: std::io::Error) -> IoDbError {
        IoDbError::Io(err)
    }
}

pub fn read_files_file(file: PathBuf) -> Result<FilesStore, IoDbError> {
    let mut fh = fs::File::open(file)?;
    let mut contents = String::new();
    fh.read_to_string(&mut contents)?;
    let file: FilesStore = toml::from_str(contents.as_str())?;
    Ok(file)
}

pub fn write_files_file(file: PathBuf, struct_to_store: FilesStore) -> Result<(), IoDbError> {
    let mut fh = fs::File::create(file)?;
    let contents = toml::to_string(&struct_to_store).expect("Couldn't serialize file store for some reason.");
    fh.write(contents.as_bytes())?;
    Ok(())
}

pub fn add_path(path: PathBuf, pipelines: Vec<String>) -> Result<(), IoDbError> {
    let mut files = read_files_file(pipe_map_path())?;
    
    let new_id = files.files.iter()
        .map(|x| x.id)
        .max()
        .unwrap_or(0) + 1;
    println!("new: {}", new_id);

    files.files.push(FileEntry { id: new_id, path: String::from(path.to_string_lossy()), pipes: pipelines });

    write_files_file(pipe_map_path(), files)
}

pub fn list_paths() -> Result<Vec<FileEntry>, IoDbError> {
    Ok(read_files_file(pipe_map_path())?.files)
}

pub fn list_pipelines() -> Result<Vec<PipelineRecord>, IoDbError> {
    let pipes: Vec<PipelineRecord> = Vec::new();
    Ok(pipes)
}

pub fn delete_path(id: u32) -> Result<(), IoDbError> {
    let mut files = read_files_file(pipe_map_path())?;

    let new_file_store = files.files.iter()
        .filter(|x| x.id != id)
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    files.files = new_file_store;
    write_files_file(pipe_map_path(), files)
}

// pub fn fetch_pipeline(id: u32) -> Result<FileEntry, IoDbError>

// pub fn list_paths<'a>() -> Result<Vec<Result<FileEntry, &'a str>>, &'a str>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn setup_works() {
        ensure_has_database();
        assert!(pipes_dir().exists(), "Directory for pipes was not created!");
    }
}
