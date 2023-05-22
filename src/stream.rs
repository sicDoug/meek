use crate::options::{ Message, Payload };
use crate::display::*;
use crate::Config;

use std::env;
use reqwest::{ Client, header::CONTENT_TYPE };
use serde_json::{ Value, from_str };

pub async fn stream(
    messages: &Vec<Message>,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {

    // get the api key
    let openai_api_key = env::var("OPENAI_API_KEY")
        .expect("No 'OPENAI_API_KEY' found in environment.");

    // construct payload
    let payload = Payload::construct(&config, &messages).unwrap();

    // varible that will hold the complete response to return
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
            print_error(raw_string.as_str());
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
            if let Ok(v) = from_str::<Value>(line) {
                // index out the content
                if let Value::String(s) = &v["choices"][0]["delta"]["content"] {
                    // write to stdout
                    print_continuous(&s);
                    response_buffer.push_str(s);
                }
            } else {
                if line.contains("[DONE]") {
                    set_color("none");
                    print!("\n\n");

                    // push the response
                    //messages.add_new(Role::Assistant, &response_buffer);
                    // write to log file
                    //write_log(&paths.log, &messages)?;
                    return Ok(response_buffer);
                }
            }
        }
    }
    Ok("error".to_string())
}
