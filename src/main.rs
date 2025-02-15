use clap::Parser;
use env_logger;
use std::error::Error;
use std::fs;
use tokio;
use extensions::file;

mod cli;
mod directory;
mod extensions;
mod json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = cli::Args::parse();

    // Read extensions.json
    if args.verbose {
        println!("Attempting to read file: {}", &args.input);
    }
    let file_content = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", &args.input, e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };
    let extensions: json::Extensions = match serde_json::from_str(&file_content) {
        Ok(extensions) => extensions,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", &args.input, e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    // Ensure the destination directory exists
    directory::create_dir_all(&args.destination)?;

    // Download each extension
    for extension in extensions.recommendations {
        if args.verbose {
            println!("Attempting to download extension: {}", &extension);
        }
        if let Err(e) = file::download(
            &extension,
            &args.destination,
            args.no_cache,
            args.proxy.as_deref(),
            args.verbose,
            args.arch.as_deref(),
        )
        .await
        {
            eprintln!("Error occurred when downloading {}: {}", extension, e);
        }
    }

    Ok(())
}
