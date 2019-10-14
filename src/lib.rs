pub mod store;
use std::process::Command;
use std::str;
// use std::path;
use std::fs;
use std::io::Write;

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
    let paths = store::list_paths()
	.expect("Was not able to get a list of paths for some reason.");

    for path in paths {
	match path {
	    Ok(store::FileEntry {id, path, cmds}) => println!("{}\t{}\t{:?}", id, path, cmds),
	    Err(e) => println!("Could not display path: {:?}", e)
	};
    }
}

pub fn delete_pipe(id: u32) {
    println!("Deleting path {:?}", id);
    store::delete_path(id);
}

pub fn run_pipeline(id: u32) {
    let pipe = store::fetch_pipeline(id)
	.expect("Unable to fetch pipeline details");

    println!("Pipe: {:?}", pipe);

    let mut fh = fs::File::create("/tmp/sup_cmd")
        .expect("Couldn't open tmp cmd file");

    for {
    }
    pipe.cmds.iter().map(|cmd| {
        write!(fh, "{}", cmd);
    });

    let output = Command::new("whoami")
        // .env("SUP_SRC", pipe.path)
        // .args(&["echo", "hello world"])
        // .args(&pipe.cmds)
        .output()
        .expect("Failed to execute process");

    println!("STDOUT: {:?}\nSTDERR: {:?}",
             str::from_utf8(&output.stdout).unwrap(),
             str::from_utf8(&output.stderr).unwrap());

    println!("Finished running");
}
