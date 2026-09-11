mod objects;
mod repository;

use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: Rgit <command>");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            repository::init(Path::new("."))?;
            println!("Initialized empty Rgit repository");
        }
        "hash-object" => {
            if args.len() < 3 {
                eprintln!("Usage: Rgit hash-object <file>");
                return Ok(());
            }

            let content = fs::read(&args[2])?;
            println!("{}", objects::blob_hash(&content));
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }

    Ok(())
}
