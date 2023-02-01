mod input;

use input::cli::Cli;

fn main() {
    let mut cli = Cli::new();

    println!("{:?}", cli.get_args().directory.as_ref().unwrap())
}
