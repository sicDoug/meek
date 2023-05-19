use crate::disk::Config;
use serde::{ Serialize, Deserialize };

pub enum Role {
    User,
    Assistant,
}

impl Role {
    pub fn in_string(&self) -> String {
        match *self {
            Role::User      => "user".to_string(),
            Role::Assistant => "assistant".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role:    String,
    pub content: String,
}

pub trait Push {
    fn add_new(&mut self, role: Role, content: &String);
}

impl Push for Vec<Message> {
    fn add_new(&mut self, role: Role, content: &String) {
        self.push(Message {
            role: role.in_string(),
            content: content.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub model:       String,
    pub stream:      bool,
    pub temperature: f32,
    pub messages:    Vec<Message>,
}

impl Payload {
    pub fn construct(config: &Config, messages: &Vec<Message>) -> String {
        serde_json::to_string(&Payload {
            model:       config.model.to_owned(),
            temperature: config.temperature.to_owned(),
            messages:    messages.to_vec(),
            stream:      true,
        }).unwrap()
    }
}
