mod cli;
use std::env;

use cli::Cli;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cli = Cli::new(args);

    println!("{:?}", cli.key_value_pairs())
}
