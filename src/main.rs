use std::path::PathBuf;
use structopt::StructOpt;
use super_pipe as sup;
use sup::store as store;

#[derive(Debug, StructOpt)]
#[structopt(about = "Super Pipelines for your filesystem")]
enum Sup {
    /// Ensure config files are in place
    Init,

    /// Add path watchers and pipelines
    Add(AddPathPipe),

    /// Manually fire all (or one if specified) pipelines
    Run {
        id: Option<u32>
    },

    /// Remove a path watcher or pipeline
    Delete(DeletePathPipe),

    /// List paths and pipelines
    List(PathsPipes),

    /// Configure super pipe
    Config(Config)
}

#[derive(Debug, StructOpt)]
enum AddPathPipe {
    /// Add a path watcher. <pipelines...> should be a list of either
    /// ids or pipeline names. These will be run when <path> changes.
    Path {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        pipelines: Vec<String>
    },
    /// Add a new pipeline. <name> should be a unique name to give
    /// this pipeline. This program then reads from STDIN and saves it
    /// to a file. You can then reference this pipeline by name or ID.
    Pipe {
        name: String
    }
}

#[derive(Debug, StructOpt)]
enum DeletePathPipe {
    /// Delete a path watcher. This does *not* remove the pipelines
    /// that are associated with the path.
    Path {
        id: u32,
    },
    /// Delete a pipe. This will not delete or modify the path watcher
    /// that reference this pipeline. Instead, a warning will be
    /// triggered when they run.
    Pipe {
        name: String
    }
}


#[derive(Debug, StructOpt)]
enum PathsPipes {
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
        // Sup::Run { id } => {
        //     match id {
        //         Some(num) => sup::run_pipeline(num),
        //         None => println!("Run all pipelines not implemented yet!")
        //     }
        // },
        // Sup::Add { path, commands: cmds } => {
        //     sup::add_path(path, cmds)
        // },
        // Sup::Delete { id } => {
        //     sup::delete_path(id)
        // },
        Sup::List(what) => {
            match what {
                PathsPipes::Paths => sup::list_paths(),
                PathsPipes::Pipelines => sup::list_pipes(),
            }
        },
        Sup::Add(what) => {
            match what {
                AddPathPipe::Path { path, pipelines } => sup::add_path(path, pipelines),
                AddPathPipe::Pipe { name: _ } => panic!("Unimplemented!")
            }
        },
        _ => panic!("Unmatched pattern: {:?}", opt)
    };
}
