use crate::options::*;
use crate::display::*;
use crate::disk::*;

use std::env;
use reqwest::{ Client, header::CONTENT_TYPE };
use serde_json::{ Value, from_str };

pub async fn start_stream(
    paths: &Paths,
    config: &Config,
    prompt: String
) -> Result<(), Box<dyn std::error::Error>> {

    // get the api key
    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("No 'OPENAI_API_KEY' found in environment.");

    // load chat log
    let mut messages = load_log(&paths.log)?;

    // push the new prompt
    messages.add_new(Role::User, &prompt);

    // construct payload
    let payload = Payload::construct(&config, &messages).unwrap();

    // varible that will hold the complete response
    let mut response_buffer = "".to_string();

    // set output color
    set_color(&config.color);
    print!("\n");

    // POST and start stream
    let mut stream = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
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
            write_error(raw_string.as_str());
            continue;
        };

        // handle each line since each chunk can contain multiple objects
        let lines = raw_string
            .lines()
            .filter_map(|line| {
                if line.starts_with("data: ") {
                    Some(&line[5..])
                } else {
                    None
                }
            });

        for line in lines {

            // parse JSON from str
            match from_str::<Value>(line) {
                Ok(v)  => {
                    // index out the content
                    if let Value::String(s) = &v["choices"][0]["delta"]["content"] {
                        // write to stdout
                        write_continuous(&s);
                        response_buffer.push_str(s);
                    }
                },
                Err(_) => {
                    if line.contains("[DONE") {
                        print!("\n\n");
                        // push the response
                        messages.add_new(Role::Assistant, &response_buffer);
                        // write to log file
                        write_log(&paths.log, &messages)?;
                    }
                },
            }
        }
    }
    Ok(())
}
