mod args;
mod stream;
mod disk;
mod options;
mod display;

use crate::{
    args::Args,
    stream::stream,
    disk::*,
    options::{ Role, Push },
};

use std::fs;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let paths = Paths::get()?;

    let args = Args::parse();

    // clear the chat
    if args.clear {
        if let Ok(()) = clear_log(&paths.log) {
            println!("\nHistory file deleted\n");
        } else {
            println!("\nNo file to delete\n");
        }
    }

    if args.reset {
        create_config(&paths.config)?;
        print!("\nConfig file reset\n");
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

    // return early if no non-flag args nor input file were given
    if prompt.is_empty() {
        return Ok(());
    }

    // load chat log
    let mut messages = load_log(&paths.log)?;

    if args.system {
        // push system message to vec
        messages.add_new(Role::System, &prompt);

        // write to log file
        write_log(&paths.log, &messages)?;

        print!("\nSystem message added to log file\n");
    } else {
        // push the new prompt
        messages.add_new(Role::User, &prompt);

        // load and map config file
        let config = load_config(&paths.config)?;

        // run the stream and get the complete response
        let response = stream(&messages, &config).await?;

        // push the response
        messages.add_new(Role::Assistant, &response);

        // write to log file
        write_log(&paths.log, &messages)?;
    }

    Ok(())
}
