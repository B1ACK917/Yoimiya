use std::collections::HashMap;

use anyhow::Result;
use reqwest::Client;

#[derive(Debug)]
pub struct Bark {
    bark_notify_url: String,
    bark_notify_args: HashMap<&'static str, String>,
    client: Client,
}

impl Bark {
    pub fn new() -> Self {
        Self {
            bark_notify_url: format!("https://api.day.app/push"),
            bark_notify_args: HashMap::new(),
            client: Client::new(),
        }
    }

    pub fn device_key(mut self, value: String) -> Bark {
        self.bark_notify_args.insert("device_key", value);
        self
    }

    pub fn title(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("title", value);
        self
    }

    pub fn body(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("body", value);
        self
    }

    pub fn group(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("group", value);
        self
    }

    pub fn badge(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("badge", value);
        self
    }

    pub fn sound(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("sound", value);
        self
    }

    pub fn icon(&mut self, value: String) -> &mut Bark {
        self.bark_notify_args.insert("icon", value);
        self
    }

    pub async fn run(&mut self) -> Result<()> {
        self.client.post(&self.bark_notify_url)
            .json(&self.bark_notify_args)
            .send()
            .await?;
        Ok(())
    }
}