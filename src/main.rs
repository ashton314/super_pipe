use std::path::PathBuf;
use structopt::StructOpt;
// use super_pipe as sup;

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
    }
}

fn main() {
    let opt = Sup::from_args();
    println!("{:?}", opt);
}

