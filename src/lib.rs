pub mod store;
use std::path::PathBuf;
use std::collections::HashMap;
use std::process::Command;
//use std::{str,fs};
//use std::io::Write;

pub fn add_path(path: PathBuf, pipes: Vec<String>) {
    println!("Adding path: {:?}, pipelines: {:?}", path, pipes);
    match store::add_path(path, pipes) {
        Ok(_) => println!("Done."),
        Err(e) => {
            println!("There was a problem:");
            match e {
                store::IoDbError::Db(_) => println!("Error from DB"),
                _ => println!("\n{:?}", e)
            }
        }
    };
}

pub fn delete_path(id: u32) {
    println!("Deleting path {:?}", id);
    match store::delete_path(id) {
        Ok(_) => println!("Done."),
        Err(e) => {
            println!("There was a problem:");
            match e {
                store::IoDbError::Db(_) => println!("Error from DB"),
                _ => println!("\n{:?}", e)
            }
        }
    };
}

pub fn list_paths() {
    println!("Known paths:");
    let paths = store::list_paths()
	.expect("Was not able to get a list of paths for some reason.");

    for path in paths {
	match path {
	    store::FileRecord {id, path, pipes} => println!("{}\t{}\t{:?}", id, path, pipes),
	};
    }
}

pub fn run_path(id: u32) {
    // First, get a path
    
}

pub fn list_pipes() {
    println!("Pipelines:");
    let pipes = store::list_pipelines()
	.expect("Was not able to get a list of pipelines for some reason.");

    for pipe in pipes {
	println!("{}\t{}", pipe.name, pipe.checksum)
    }
}

pub fn add_pipe(name: String, content: String) {
    match store::add_pipeline(name, content) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Problem adding pipeline: {:?}", e),
    };
}

/// Given a FileEntry, run all it's pipelines
pub fn run_pipeline(name: &String, env: HashMap<String, String>) {
    let record = match store::fetch_pipeline(name) {
        Ok(thing) => match thing {
            Some(r) => r,
            None => {
                println!("No pipeline matching name {}", name);
                return
            }
        },
        Err(e) => {
            println!("Problem fetching record: {:?}", e);
            return
        }
    };

    let mut pipeline = store::pipes_dir();
    pipeline.push(name);
    if ! pipeline.exists() {
	println!("Could not locate pipeline {} (hash: {})", name, record.checksum);
        return
    }

    Command::new("bash")
        .args(&[pipeline])
        .envs(&env)
        .spawn()
        .expect(format!("Problem running pipeline {}", name).as_str());
}

// pub fn run_pipeline(id: u32) {
//     let pipe = store::fetch_pipeline(id)
// 	.expect("Unable to fetch pipeline details");

//     println!("Pipe: {:?}", pipe);

//     let mut fh = fs::File::create("/tmp/sup_cmd")
//         .expect("Couldn't open tmp cmd file");

//     for cmd in pipe.pipes.iter() {
// 	write!(fh, "{}", cmd).unwrap();
//     }

//     let output = Command::new("bash")
//         .env("SUP_SRC", pipe.path)
// 	.args(&["/tmp/sup_cmd"])
//         .output()
//         .expect("Failed to execute process");

//     println!("STDOUT: {:?}\nSTDERR: {:?}",
//              str::from_utf8(&output.stdout).unwrap(),
//              str::from_utf8(&output.stderr).unwrap());

//     println!("Finished running");
// }
