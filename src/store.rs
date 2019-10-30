extern crate serde;
extern crate toml;
extern crate dirs;
extern crate sha1;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
use sha1::Sha1;

#[derive(Serialize, Deserialize, Debug)]
pub struct FilesStore {
    files: Vec<FileRecord>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineStore {
    pipes: Vec<PipelineRecord>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileRecord {
    pub id: u32,
    pub path: String,
    pub pipes: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineRecord {
    pub name: String,
    pub checksum: String
}

/// Return the root of the directory that we can own
pub fn conf_dir() -> PathBuf {
    let mut path = dirs::config_dir()
        .expect("Can't find user's configuration directory.");
    path.push("super_pipe");
    path
}

/// Returns the directory where all pipes are stored
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

/// This returns the path to the file that keeps track of all the pipelines
pub fn pipe_idx_path() -> PathBuf {
    let mut path = conf_dir();
    path.push("pipelines");
    path.set_extension("toml");
    path
}

/// Makes sure that there is a file store available
pub fn ensure_has_database() {
    let conf  = conf_dir();
    let pipes = pipes_dir();
    let fmp   = pipe_map_path();
    let psp   = pipe_idx_path();
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
    if ! psp.exists() {
        fs::File::create(&psp)
            .expect(format!("Could not create new pipe store path at {:?} for some eason", psp).as_str());
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

impl From<std::io::Error> for IoDbError {
    fn from(err: std::io::Error) -> IoDbError {
        IoDbError::Io(err)
    }
}

// TODO: make this function accept a mutable destination of type FilesStore or PipelinesStore... or can I pass a *type* as a value?
pub fn read_files_file(file: PathBuf) -> Result<FilesStore, IoDbError> {
    let mut fh = fs::File::open(file)?;
    let mut contents = String::new();
    fh.read_to_string(&mut contents)?;
    if contents.len() == 0 {
	Ok(FilesStore {files: Vec::new()})
    } else {
	let file: FilesStore = toml::from_str(contents.as_str())?;
	Ok(file)
    }
}

pub fn read_pipes_file(file: PathBuf) -> Result<PipelineStore, IoDbError> {
    let mut fh = fs::File::open(file)?;
    let mut contents = String::new();
    fh.read_to_string(&mut contents)?;
    if contents.len() == 0 {
	Ok(PipelineStore { pipes: Vec::new()})
    } else {
	let file: PipelineStore = toml::from_str(contents.as_str())?;
	Ok(file)
    }
}

pub fn write_struct_to_file(file: PathBuf, struct_to_store: impl Serialize) -> Result<(), IoDbError> {
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

    files.files.push(FileRecord { id: new_id, path: String::from(path.to_string_lossy()), pipes: pipelines });

    write_struct_to_file(pipe_map_path(), files)
}

pub fn list_paths() -> Result<Vec<FileRecord>, IoDbError> {
    Ok(read_files_file(pipe_map_path())?.files)
}

pub fn list_pipelines() -> Result<Vec<PipelineRecord>, IoDbError> {
    Ok(read_pipes_file(pipe_idx_path())?.pipes)
}

pub fn delete_path(id: u32) -> Result<(), IoDbError> {
    let mut files = read_files_file(pipe_map_path())?;

    let new_file_store = files.files.iter()
        .filter(|x| x.id != id)
        .map(|x| x.clone())
        .collect::<Vec<_>>();

    files.files = new_file_store;
    write_struct_to_file(pipe_map_path(), files)
}

/// Given a name and some contents, stores the contents under it's
/// checksum and creates a new pipeline record and stores that.
pub fn add_pipeline(name: String, contents: String) -> Result<(), IoDbError> {
    let mut pipes = read_pipes_file(pipe_idx_path())?;
    let checksum: String = Sha1::from(&contents).digest().to_string();
    write_to_pipe(&checksum, &contents)?;

    pipes.pipes.push(PipelineRecord { name, checksum });

    write_struct_to_file(pipe_idx_path(), pipes)
}

/// write_to_pipe just spews out data to a file in the pipes_dir()
fn write_to_pipe(checksum: &String, contents: &String) -> Result<(), IoDbError> {
    let mut source = pipes_dir(); source.push(checksum);
    let mut fh = fs::File::create(source)?;

    fh.write(contents.as_bytes())?;
    Ok(())
}

// pub fn fetch_pipeline(id: u32) -> Result<FileRecord, IoDbError>

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn setup_works() {
        ensure_has_database();
        assert!(pipes_dir().exists(), "Directory for pipes was not created!");
    }

    #[test]
    fn run_with_empty_file() {
	let tmp_file = PathBuf::from("/tmp/super_pipe_test_file_will_be_killed");
	let mut fh = fs::File::create(&tmp_file).unwrap();
	fh.write(b"").unwrap();
	assert_eq!(read_files_file(tmp_file).unwrap().files.len(), 0, "Somehow some files snuck in...")
    }
}
