use clap::Parser;
use env_logger;
use std::error::Error;
use std::fs;
use tokio;
use extensions::file;
use extensions::info as ext_info; // For info command

mod cli;
mod directory;
mod extensions;
mod json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = cli::Cli::parse();

    // Global options
    let proxy = cli.proxy.as_deref();
    let verbose = cli.verbose;

    match cli.command {
        cli::Commands::Download(args) => {
            let extensions_to_download: Vec<String>;

            if let Some(single_extension_id) = &args.single {
                if verbose {
                    println!("Attempting to download single extension: {}", single_extension_id);
                }
                extensions_to_download = vec![single_extension_id.clone()];
            } else {
                if verbose {
                    println!("Attempting to read file: {}", &args.input);
                }
                let file_content = match fs::read_to_string(&args.input) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Failed to read file {}: {}", &args.input, e);
                        return Err(Box::new(e) as Box<dyn Error>);
                    }
                };
                let extensions_data: json::Extensions = match serde_json::from_str(&file_content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to parse file {}: {}", &args.input, e);
                        return Err(Box::new(e) as Box<dyn Error>);
                    }
                };
                extensions_to_download = extensions_data.recommendations;
            }

            directory::create_dir_all(&args.destination)?;

            for extension_id_str in extensions_to_download {
                if verbose {
                    println!("Processing extension for download: {}", &extension_id_str);
                }
                if let Err(e) = file::download(
                    &extension_id_str,
                    &args.destination,
                    args.force,
                    proxy, // Use global proxy
                    verbose, // Use global verbose
                    args.arch.as_deref(),
                )
                .await
                {
                    eprintln!("Error occurred when downloading {}: {}", extension_id_str, e);
                }
            }
        }
        cli::Commands::Info(args) => {
            let extensions_to_info: Vec<String>;

            if let Some(single_extension_id) = &args.single {
                if verbose {
                    println!("Fetching info for single extension: {}", single_extension_id);
                }
                extensions_to_info = vec![single_extension_id.clone()];
            } else {
                if verbose {
                    println!("Attempting to read file for info: {}", &args.input);
                }
                let file_content = match fs::read_to_string(&args.input) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Failed to read file {}: {}", &args.input, e);
                        return Err(Box::new(e) as Box<dyn Error>);
                    }
                };
                let extensions_data: json::Extensions = match serde_json::from_str(&file_content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to parse file {}: {}", &args.input, e);
                        return Err(Box::new(e) as Box<dyn Error>);
                    }
                };
                extensions_to_info = extensions_data.recommendations;
            }

            for (index, extension_id_str) in extensions_to_info.iter().enumerate() {
                if verbose {
                    println!("Processing info for extension: {}", &extension_id_str);
                }
                if index > 0 { // Add a separator for multiple extensions
                    println!("--------------------");
                }

                let parsed_name = ext_info::parse_extension_name(extension_id_str);
                match ext_info::get(&parsed_name.publisher, &parsed_name.name, proxy, verbose).await {
                    Ok(info) => {
                        println!("Extension: {}.{}", parsed_name.publisher, parsed_name.name);
                        if info.arch_versions.is_empty() {
                            println!("  No version information found.");
                        } else {
                            println!("  Available versions per platform:");
                            let mut sorted_versions: Vec<_> = info.arch_versions.iter().collect();
                            sorted_versions.sort_by_key(|(platform, _)| platform.as_deref().unwrap_or("universal"));

                            for (platform_opt, version) in sorted_versions {
                                let platform_str = platform_opt.as_deref().unwrap_or("Platform Independent");
                                println!("    - {}: {}", platform_str, version);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get info for {}: {}", extension_id_str, e);
                        // Continue to the next extension if one fails in multi-mode
                        if args.single.is_some() {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
