use std::path::PathBuf;
use structopt::StructOpt;
use super_pipe as sup;
use sup::store as store;

#[derive(Debug, StructOpt)]
#[structopt(about = "Super Pipelines for your filesystem")]
enum Sup {
    /// Ensure config files are in place
    Init,
    /// Add a path watcher
    Add_path {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        pipelines: Vec<String>
    },
    /// Add a new pipeline. <name> should be a unique name to give this pipeline. This program then reads from STDIN and saves it to a file. You can then 
    Add_pipeline {
	name: String
    },
    /// Manually fire all (or one if specified) pipelines
    Run {
        id: Option<u32>
    },
    /// Remove a pipeline
    Delete {
        id: u32
    },
    /// List paths and pipelines
    List(PathPipe),
    /// Configure super pipe
    Config(Config)
}

#[derive(Debug, StructOpt)]
enum PathPipe {
    /// Display paths and their associated pipelines
    Paths,
    /// Display pipelines
    Pipelines
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
        Sup::Run { id } => {
            match id {
                Some(num) => sup::run_pipeline(num),
                None => println!("Run all pipelines not implemented yet!")
            }
        },
        Sup::Add { path, commands: cmds } => {
            sup::add_path(path, cmds)
        },
        Sup::Delete { id } => {
            sup::delete_pipe(id)
        },
        Sup::List => sup::list_paths(),
        _ => panic!("Unmatched pattern: {:?}", opt)
    };
}
