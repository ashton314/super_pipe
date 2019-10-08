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
    Add {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        commands: Vec<String>
    },
    /// Manually fire all (or one if specified) pipelines
    Run {
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>
    },
    /// Remove a pipeline
    Delete {
        id: u32,
    },
    /// List all paths and pipelines
    List,
    /// Configure super pipe
    Config(Config)
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

    store::init();

    // Dispatch on the sub-commands
    match opt {
        Sup::Init => {
            panic!("Don't know how to resolve the home directory")
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
