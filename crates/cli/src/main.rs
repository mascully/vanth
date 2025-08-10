use clap::Parser;

mod cli;

pub use cli::*;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let cli = Cli::parse();
    cli::execute(cli);
}
