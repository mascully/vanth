use clap::Parser;

mod cli;

use cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli::execute(cli);
}
