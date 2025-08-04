use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "vanth")]
#[command(about = "Vanth CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "File system operations")]
    Fs {
        #[command(subcommand)]
        command: FsCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum FsCommands {
    #[command(about = "Read from path and print JSON to stdout")]
    Open {
        #[arg(help = "Path to read from")]
        path: String,
    },
    #[command(about = "Take JSON from stdin and write to path")]
    Save {
        #[arg(help = "Path to write to")]
        path: String,
    },
}

pub fn execute(cli: Cli) {
    match cli.command {
        Commands::Fs { command } => match command {
            FsCommands::Open { path } => {
                // TODO: implement fs open functionality
                println!("fs open: {}", path);
            }
            FsCommands::Save { path } => {
                // TODO: implement fs save functionality
                println!("fs save: {}", path);
            }
        },
    }
}