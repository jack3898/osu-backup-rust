mod input;

use input::cli::Cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cli = Cli::new(args);

    println!("{:?}", cli.get_args().directory)
}
