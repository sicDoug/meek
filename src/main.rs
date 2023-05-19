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

    let paths = Paths::get();

    // clear the chat
    // TODO make this independent of other args
    if Args::parse().clear {
        clear_log(&paths.log);
        return Ok(());
    }

    // collect all non-flag args in vector of strings
    let mut prompt = Args::parse().prompt.join(" ");

    // appends potential input file to prompt
    if let Some(input_path) = Args::parse().input {
        let contents = fs::read_to_string(&input_path)
            .expect("this needs handling");
        let file_name = input_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        prompt += &format!("\n\n{}\n```\n{}```", file_name, contents);
    }

    let config = load_config(&paths.config);

    start_stream(&paths, &config, prompt).await.unwrap();

    Ok(())
}
