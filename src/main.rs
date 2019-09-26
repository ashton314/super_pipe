use structopt::StructOpt;
use super_pipe;

// Setup command line argument options
#[derive(StructOpt)]
struct Cli {
    command: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    println!("Hello, world!");
    let args = Cli::from_args();
    println!("Command: {}", args.command);
    println!("Path: {:?}", args.path);
    super_pipe::foo();
}
