use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>, // Made subcommand optional

    #[clap(flatten)]
    pub download_defaults: DownloadArgs, // Options for default (no subcommand) download

    /// Specify proxy url for all commands
    #[clap(long, global = true)]
    pub proxy: Option<String>,

    /// Show verbose infomation for all commands
    #[clap(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download VSIX extension(s)
    Download(DownloadArgs),
    /// Show information for a VSIX extension
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
pub struct DownloadArgs {
    /// Path to extensions.json
    #[arg(short, long, default_value = "./.vscode/extensions.json")]
    pub input: String,

    /// Output directory
    #[arg(short, long, default_value = "./.vscode/extensions")]
    pub destination: String,

    /// Force redownload if exists
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Specify OS architecture for downloaded extensions
    #[arg(short = 'a', long, value_name = "ARCHITECTURE")] // Changed short name to avoid conflict if -s is used globally
    pub arch: Option<String>,

    /// Download a single extension by its ID (e.g., publisher.extensionName)
    /// If used, --input is ignored.
    #[arg(short = 's', long, value_name = "EXTENSION_ID")]
    pub single: Option<String>,
}

#[derive(Parser, Debug)]
pub struct InfoArgs {
    /// Path to extensions.json (used if --single is not provided)
    #[arg(short, long, default_value = "./.vscode/extensions.json")]
    pub input: String,

    /// Show info for a single extension by its ID (e.g., publisher.extensionName)
    /// If used, --input is ignored.
    #[arg(short = 's', long, value_name = "EXTENSION_ID")]
    pub single: Option<String>,
    // Note: proxy and verbose are global options now, inherited by subcommands.
    // If info-specific arch filtering is needed, add --arch to InfoArgs.
}

