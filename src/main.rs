use std::path::PathBuf;
use structopt::StructOpt;
use super_pipe as sup;

#[derive(Debug, StructOpt)]
#[structopt(about = "Super Pipelines for your filesystem")]
enum Sup {
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
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
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
    println!("{:?}", opt);

    // Dispatch on the sub-commands
    let foo = 
        match opt {
            Sup::Add { path, commands: cmds } => {
                println!("Path: {:?}", path);
                println!("Commands: {:?}", cmds);
                1
            },
            _ => 2
        };
    println!("{}", foo);
}
