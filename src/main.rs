use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use tokio;

mod download;
mod version;

#[derive(Parser)]
struct Args {
    /// Path to extensions.json
    #[arg(short, long, default_value = "./.vscode/extensions.json")]
    input: String,

    /// Output directory
    #[arg(short, long, default_value = "./.vscode/extensions")]
    destination: String,

    /// Force redownload if exists
    #[arg(long)]
    no_cache: bool,

    /// Specify proxy url
    #[arg(long)]
    proxy: Option<String>,

    /// Show verbose infomation
    #[arg(short, long)]
    verbose: bool,

    /// Specify OS architecture
    #[arg(short, long)]
    arch: Option<String>,
}

#[derive(Deserialize)]
struct Extensions {
    recommendations: Vec<String>,
}

fn create_directory_if_not_exists(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

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
    let extensions: Extensions = match serde_json::from_str(&file_content) {
        Ok(extensions) => extensions,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", &args.input, e);
            return Err(Box::new(e) as Box<dyn Error>);
        }
    };

    // Ensure the destination directory exists
    create_directory_if_not_exists(&args.destination)?;

    // Download each extension
    for extension in extensions.recommendations {
        if args.verbose {
            println!("Attempting to download extension: {}", &extension);
        }
        if let Err(e) = download::download_extension(
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
