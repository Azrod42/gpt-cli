use colored::Colorize;

use crate::{
    file,
    structs::{
        cli::Cli,
        gpt::{Gpt, GptResponseUsage},
    },
};

pub mod default_completion;

pub struct Client {
    client: reqwest::Client,
    pub gpt: Gpt,
    pub server: String,
    pub chat_history_fn: String,
    pub usage_log_fn: String,
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Client {
    const DEFAULT_SERVER: &'static str = "https://api.openai.com";

    pub fn new(openai_key: String, args: Cli) -> Self {
        Client {
            client: reqwest::Client::new(),
            gpt: Gpt::new(openai_key, args),
            server: Self::DEFAULT_SERVER.to_string(),
            chat_history_fn: String::from("/gpt-cli-m-h.log"),
            usage_log_fn: String::from("/gpt-cli-u-h.log"),
        }
    }

    pub fn print_usage(&self) {
        let history = file::read_tmp_file::<GptResponseUsage>(&self.usage_log_fn);
        match history {
            Some(data) => print!("{}", data),
            None => println!("{}: Usage log file not found.", "Error".red()),
        }
    }
}
