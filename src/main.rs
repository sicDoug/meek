mod args;
mod options;
mod display;
mod disk;
mod stream;

use crate::{
    args::Args,
    stream::start_stream,
    disk::Paths,
    disk::clear_log,
    disk::load_config,
};

use std::fs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let paths = Paths::get()?;

    let args = Args::parse();

    // clear the chat
    // TODO make this independent of other args
    if args.clear {
        if let Ok(()) = clear_log(&paths.log) {
            println!("\nHistory file deleted\n");
        } else {
            println!("\nNo file to delete\n");
        }
    }

    // collect all non-flag args to string
    let mut prompt = args.prompt.join(" ");

    // appends potential input file to prompt
    if let Some(input_path) = args.input {
        let contents = fs::read_to_string(&input_path)?;

        let file_name = input_path
            .to_str()
            .unwrap();
        prompt += &format!("\n\n{}\n```\n{}```", file_name, contents);
    }

    if prompt.is_empty() {
        return Ok(());
    }

    let config = load_config(&paths.config)?;

    start_stream(&paths, &config, prompt).await?;

    Ok(())
}
