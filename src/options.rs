use crate::disk::Config;
use serde::{ Serialize, Deserialize };

pub enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    pub fn get_str(&self) -> &'static str {
        match *self {
            Role::System     => "system",
            Role::User      => "user",
            Role::Assistant => "assistant",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role:    String,
    pub content: String,
}

pub trait Push {
    fn add_new(&mut self, role: Role, content: &str);
}

impl Push for Vec<Message> {
    fn add_new(&mut self, role: Role, content: &str) {
        self.push(Message {
            role: role.get_str().to_owned(),
            content: content.to_owned(),
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
    pub fn construct(
        config: &Config,
        messages: &[Message]
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&Payload {
            model:       config.model.to_owned(),
            temperature: config.temperature.to_owned(),
            messages:    messages.to_vec(),
            stream:      true,
        })
    }
}
