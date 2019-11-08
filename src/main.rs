use std::path::PathBuf;
use std::io::{self, Read};
use structopt::StructOpt;
use super_pipe as sup;
use sup::store as store;

#[derive(Debug, StructOpt)]
#[structopt(about = "Super Pipelines for your filesystem")]
enum Sup {
    /// Ensure config files are in place
    Init,

    /// Path-related commands
    Path(PathCommands),

    /// Pipeline-related commands
    Pipe(PipeCommands),

    /// Manually fire all (or one if specified) pipelines
    Run,

    /// Watch all paths for changes
    Watch,

    /// Configure super pipe
    Config(Config)
}

#[derive(Debug, StructOpt)]
enum PathCommands {
    /// Add a path watcher. <pipelines...> should be a list of
    /// pipeline names. These will be run when <path> changes.
    Add {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        pipelines: Vec<String>
    },
    
    /// List all path watchers.
    List,

    /// Run pipelines associated with a file (or all files if none specified)
    Run {
	id: Option<u32>,
    },

    /// Delete a path watcher. This does *not* remove the pipelines
    /// that are associated with the path.
    Delete {
        id: u32
    }
}

#[derive(Debug, StructOpt)]
enum PipeCommands {
    /// Add a new pipeline. <name> should be a unique name to give
    /// this pipeline. This program then reads from STDIN and saves it
    /// to a file. You can then reference this pipeline by name or ID.
    Add {
        name: String
    },

    /// List all pipelines
    List,

    /// Delete a pipe. This will not delete or modify the path watcher
    /// that reference this pipeline. Instead, a warning will be
    /// triggered when they run.
    Delete {
        name: String
    }
}

#[derive(Debug, StructOpt)]
enum Config {
    /// Display a listing of configuration values
    List,
    /// Set a configuration value
    Set {
        key: String,
        value: String
    },
    /// Display a single configuration value
    Get {
        key: String
    }
}

fn main() {
    let opt = Sup::from_args();

    // TODO: add pre-flight check

    // Dispatch on the sub-commands
    match opt {
        Sup::Init => {
            print!("Initilizing super pipe... ");
            store::ensure_has_database();
            store::init();
            println!("done.")
        },
        Sup::Path(what) => {
            match what {
                PathCommands::Add { path, pipelines } => sup::add_path(path, pipelines),
                PathCommands::List => sup::list_paths(),
                PathCommands::Delete { id } => sup::delete_path(id),
		PathCommands::Run { id: maybe_id } => match maybe_id {
		    None => println!("Running all paths not yet implemented!"),
		    Some(id) => sup::run_path(id)
		}
            }
        },
        Sup::Pipe(what) => {
            match what {
                PipeCommands::Add { name } => {
		    println!("Reading file from STDIN...");
		    let mut contents = String::new();
		    io::stdin().read_to_string(&mut contents).expect("Couldn't read STDIN");
		    sup::add_pipe(name, contents)
		},
		PipeCommands::List => sup::list_pipes(),
		_ => panic!("Unimplemented pattern: {:?}", what)
            }
        },
        Sup::Watch => {
            println!("Firing off file system watcher...");
            sup::watch()
        },
        _ => panic!("Unmatched pattern: {:?}", opt)
    };
}
