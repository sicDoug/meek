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
use std::process;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let paths = Paths::get()
        .unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(1);
        });

    let args = Args::parse();

    // clear the chat
    if args.clear {
        if let Ok(()) = clear_log(&paths.log) {
            println!("History file deleted");
        } else {
            println!("No file to delete");
        }
    }

    if args.reset {
        match create_config(&paths.config) {
            Ok(()) => println!("Config file reset"),
            Err(e) => eprintln!("{}", e),
        }
    }

    // collect vec of non-flag args to string
    let mut prompt = args.prompt.join(" ");

    // appends potential input file to prompt
    if let Some(input_path) = args.input {
        let contents = fs::read_to_string(&input_path).unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
        });

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
    let mut messages = load_log(&paths.log).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    if args.system {
        // push system message to vec
        messages.add_new(Role::System, &prompt);

        // write to log file
        write_log(&paths.log, &messages).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });

        print!("System message added to log file");
    } else {
        // push the new prompt
        messages.add_new(Role::User, &prompt);

        // load and map config file
        let config = load_config(&paths.config).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });

        // run the stream and get the complete response
        let response = stream(&messages, &config).await?;

        // push the response
        messages.add_new(Role::Assistant, &response);

        // write to log file
        write_log(&paths.log, &messages).unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(1);
        });
    }

    Ok(())
}
