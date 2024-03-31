use crate::structs::gpt::Gpt;

pub mod correction;
pub mod default_completion;
pub mod translation;

pub struct Client {
    client: reqwest::Client,
    pub gpt: Gpt,
    pub server: String,
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Client {
    const DEFAULT_SERVER: &'static str = "https://api.openai.com";

    pub fn new(openai_key: String, usage: bool) -> Self {
        Client {
            client: reqwest::Client::new(),
            gpt: Gpt::new(openai_key, usage),
            server: Self::DEFAULT_SERVER.to_string(),
        }
    }
}
