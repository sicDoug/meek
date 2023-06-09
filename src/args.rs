use clap::Parser;

/// ChatGPT CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Prompt
    #[clap(name = "prompt")]
    pub prompt: Vec<String>,
    /// Input file
    #[clap(short, long)]
    pub input: Option<std::path::PathBuf>,
    /// Delete chat history
    #[clap(short, long)]
    pub clear: bool,
    /// Add System message
    #[clap(short, long)]
    pub system: bool,
    /// Reset config
    #[clap(long)]
    pub reset: bool,
    #[clap(short, long)]
    pub list: bool,
    /// Do not stream
    #[clap(long)]
    pub no_stream: bool,
}
