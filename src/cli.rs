use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to extensions.json
    #[arg(short, long, default_value = "./.vscode/extensions.json")]
    pub input: String,

    /// Output directory
    #[arg(short, long, default_value = "./.vscode/extensions")]
    pub destination: String,

    /// Force redownload if exists
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Specify proxy url
    #[arg(long)]
    pub proxy: Option<String>,

    /// Show verbose infomation
    #[arg(short, long)]
    pub verbose: bool,

    /// Specify OS architecture
    #[arg(short, long)]
    pub arch: Option<String>,

    /// Download a single extension by its ID (e.g., publisher.extensionName)
    #[arg(short, long, value_name = "EXTENSION_ID")]
    pub single: Option<String>,
}
