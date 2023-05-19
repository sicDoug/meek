use crate::options::*;
use crate::display::*;
use crate::disk::*;

use std::env;
use reqwest::{ Client, header::CONTENT_TYPE };
use serde_json::{ Value, from_str };

pub async fn start_stream(paths: &Paths, config: &Config, prompt: String) -> Result<(), Box<dyn std::error::Error>> {

    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("No 'OPENAI_API_KEY' found in environment.");

    let mut messages = load_log(&paths.log);
    messages.add_new(Role::User, &prompt);

    let payload = Payload::construct(&config, &messages);

    let mut res_buffer = "".to_string();

    print!("\n");

    let mut stream = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(openai_api_key)
        .header(CONTENT_TYPE, "application/json")
        .body(payload)
        .send()
        .await?;

    while let Some(chunk) = stream.chunk().await? {

        // convert byte slice to string
        let raw_string = String::from_utf8(chunk.to_vec())
            .unwrap();

        // errors do not start with 'data: '
        if raw_string.starts_with("{") {
            write_error(raw_string.as_str());
            continue;
        };

        // handle each line since each chunk can contain multiple objects
        let lines = raw_string
            .split("\n")
            .filter(|line| line.starts_with("data: "));

        for line in lines {
            match from_str::<Value>(&line[5..]) {
                Ok(v)  => {
                    if let Value::String(s) = &v["choices"][0]["delta"]["content"] {
                        res_buffer += &s;
                        write_continuous(&config.color.as_str(), &s);
                    }
                },
                // TODO make certain that this is 'DONE'
                Err(_) => {
                    print!("\n\n");
                    messages.add_new(Role::Assistant, &res_buffer);
                    write_log(&paths.log, &messages);
                },
            }
        }
    }
    Ok(())
}
