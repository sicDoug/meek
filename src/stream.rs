use crate::options::{ Message, Payload };
use crate::display::*;
use crate::Config;

use std::env;
use std::io;
use std::io::Write;
use reqwest::{ Client, header::CONTENT_TYPE };
use serde_json::{ Value, from_str };

const OPENAI_API_URL:   &str = "https://api.openai.com/v1/chat/completions";
const DATA_PREFIX:      &str = "data: ";
const DONE_MARKER:      &str = "[DONE]";

pub async fn stream(
    messages: &Vec<Message>,
    config:   &Config,
) -> Result<String, Box<dyn std::error::Error>> {

    // get the api key
    let openai_api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        eprintln!("No 'OPENAI_API_KEY' found in environment.");
        io::Error::new(io::ErrorKind::Other, "Missing OPENAI_API_KEY environment variable")
    })?;

    // construct payload
    let payload = Payload::construct(config, messages)?;


    // varible that will hold the complete response to return
    let mut full_response = String::new();

    // set output color
    set_color(&config.color);

    print!("\n");

    // POST and start stream
    let mut stream = Client::new()
        .post(OPENAI_API_URL)
        .bearer_auth(openai_api_key)
        .header(CONTENT_TYPE, "application/json")
        .body(payload)
        .send()
        .await?;

    // handle each chunk of incoming data
    while let Some(chunk) = stream.chunk().await? {
        // convert byte slice to string
        let raw_string = String::from_utf8(chunk.to_vec())?;

        // errors do not start with 'data: '
        if raw_string.starts_with("{") {
            set_color("none");
            print_error(raw_string.as_str());
            return Ok(raw_string);
        };

        // filter map each line starting with 'data: '
        let lines = raw_string
            .lines()
            .filter_map(|line| {
                if line.starts_with(DATA_PREFIX) {
                    Some(&line[DATA_PREFIX.len()..])
                } else {
                    None
                }
            });

        // handle each line since each chunk can contain multiple objects
        for line in lines {

            // parse JSON from str
            match from_str::<Value>(line) {
                // index out the content
                Ok(json_obj)    => {
                    if let Value::String(s) = &json_obj["choices"][0]["delta"]["content"] {
                        print!("{}", s);
                        std::io::stdout()
                            .flush()
                            .unwrap();

                        full_response.push_str(s);
                    }
                }
                Err(_)          => {
                    if line.contains(DONE_MARKER) {
                        set_color("none");
                        print!("\n\n");

                        // return the response
                        return Ok(full_response);
                    }
                }
            }
        }
    }

    // if neither 'done' nor error is received from the server
    Err(Box::new(io::Error::new(io::ErrorKind::Other, "Unexpected termination of stream")))
}
