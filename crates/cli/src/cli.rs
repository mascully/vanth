use clap::{Args, Parser, Subcommand};
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use vanth::{ContentHash, store::Store, Ty};
use vanth::hash as vanth_hash;

#[derive(Parser, Debug)]
#[command(name = "vanth")]
#[command(about = "Vanth CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Write a value to the store")]
    Write(WriteArgs),
    #[command(about = "Get a value from the store")]
    Get(GetArgs),
    #[command(about = "Get all values of a type from the store")]
    GetAll(GetAllArgs),
    #[command(about = "Delete a value by hash from all types")]
    Delete(DeleteArgs),
    #[command(about = "Delete all values of a type")]
    DeleteAll(DeleteAllArgs),
}

#[derive(Args, Debug)]
pub struct WriteArgs {
    #[arg(long, help = "Database file path")]
    db: PathBuf,
    #[arg(long, help = "Type name, e.g., path::to::Type")]
    ty: String,
    #[arg(long, help = "JSON value to write, optional (read from stdin if omitted)")]
    value: Option<String>,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    #[arg(long, help = "Database file path")]
    db: PathBuf,
    #[arg(long, help = "Type name, e.g., path::to::Type")]
    ty: String,
    #[arg(help = "Content hash as 64-character hex string")]
    content_hash: String,
}

#[derive(Args, Debug)]
pub struct GetAllArgs {
    #[arg(long, help = "Database file path")]
    db: PathBuf,
    #[arg(long, help = "Type name, e.g., path::to::Type")]
    ty: String,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    #[arg(long, help = "Database file path")]
    db: PathBuf,
    #[arg(help = "Content hash as 64-character hex string")]
    content_hash: String,
}

#[derive(Args, Debug)]
pub struct DeleteAllArgs {
    #[arg(long, help = "Database file path")]
    db: PathBuf,
    #[arg(long, help = "Type name, e.g., path::to::Type")]
    ty: String,
}

pub fn execute(cli: Cli) {
    match cli.command {
        Commands::Write(args) => handle_write(&args),
        Commands::Get(args) => handle_get(&args),
        Commands::GetAll(args) => handle_get_all(&args),
        Commands::Delete(args) => handle_delete(&args),
        Commands::DeleteAll(args) => handle_delete_all(&args),
    }
}

fn parse_ty(s: &str) -> Ty {
    Ty {
        path: s.split("::").map(|p| p.to_string()).collect(),
    }
}

fn parse_hash(s: &str) -> ContentHash {
    if s.len() != 64 {
        eprintln!("Hash must be exactly 64 hexadecimal characters");
        process::exit(1);
    }
    let mut hash = [0u8; 32];
    for (i, byte) in hash.iter_mut().enumerate() {
        let hex_slice = &s[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(hex_slice, 16).unwrap_or_else(|_| {
            eprintln!("Invalid hexadecimal in hash: {}", hex_slice);
            process::exit(1);
        });
    }
    ContentHash { hash }
}

fn handle_write(args: &WriteArgs) {
    let mut store = Store::sqlite_from_path(args.db.clone()).unwrap_or_else(|e| {
        eprintln!("Error opening store: {:?}", e);
        process::exit(1);
    });
    let ty = parse_ty(&args.ty);

    let mut content = String::new();
    if let Some(val) = &args.value {
        content = val.clone();
    } else {
        io::stdin().read_to_string(&mut content).unwrap_or_else(|e| {
            eprintln!("Error reading from stdin: {}", e);
            process::exit(1);
        });
    }

    let value: serde_json::Value = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Invalid JSON: {}", e);
        process::exit(1);
    });
    let data = serde_json::to_vec(&value).unwrap();
    let content_hash = vanth_hash(&value);

    store.write_raw(ty, content_hash, data).unwrap_or_else(|e| {
        eprintln!("Error writing to store: {:?}", e);
        process::exit(1);
    });
}

fn handle_get(args: &GetArgs) {
    let mut store = Store::sqlite_from_path(args.db.clone()).unwrap_or_else(|e| {
        eprintln!("Error opening store: {:?}", e);
        process::exit(1);
    });
    let ty = parse_ty(&args.ty);
    let content_hash = parse_hash(&args.content_hash);

    let raw = store.get_from_hash_raw(ty, content_hash).unwrap_or_else(|e| {
        eprintln!("Error getting from store: {:?}", e);
        process::exit(1);
    });
    match raw {
        Some(data) => {
            let output = String::from_utf8(data).unwrap_or_else(|e| {
                eprintln!("Invalid UTF-8 in data: {}", e);
                process::exit(1);
            });
            println!("{}", output);
        }
        None => {
            process::exit(1);
        }
    }
}

fn handle_get_all(args: &GetAllArgs) {
    let mut store = Store::sqlite_from_path(args.db.clone()).unwrap_or_else(|e| {
        eprintln!("Error opening store: {:?}", e);
        process::exit(1);
    });
    let ty = parse_ty(&args.ty);

    let items = store.get_all_of_type_raw(ty).unwrap_or_else(|e| {
        eprintln!("Error getting all from store: {:?}", e);
        process::exit(1);
    });
    for (_, data) in items {
        let output = String::from_utf8(data).unwrap_or_else(|e| {
            eprintln!("Invalid UTF-8 in data: {}", e);
            process::exit(1);
        });
        println!("{}", output);
    }
}

fn handle_delete(args: &DeleteArgs) {
    let mut store = Store::sqlite_from_path(args.db.clone()).unwrap_or_else(|e| {
        eprintln!("Error opening store: {:?}", e);
        process::exit(1);
    });
    let content_hash = parse_hash(&args.content_hash);
}

fn handle_delete_all(args: &DeleteAllArgs) {
    let mut store = Store::sqlite_from_path(args.db.clone()).unwrap_or_else(|e| {
        eprintln!("Error opening store: {:?}", e);
        process::exit(1);
    });
    let ty = parse_ty(&args.ty);

    store.delete_all_raw(ty).unwrap_or_else(|e| {
        eprintln!("Error deleting all from store: {:?}", e);
        process::exit(1);
    });
}
