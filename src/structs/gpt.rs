use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    vec,
};
use termimad::{minimad::TextTemplate, print_text, MadSkin};

use crate::{file, reqwest::Client};

use super::cli::Cli;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GptMessage {
    pub role: String,
    pub content: String,
}

impl Display for GptMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let res = write!(f, "");
        print_text(&self.content);
        res
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponseChoice {
    pub index: i32,
    pub message: GptMessage,
    pub finish_reason: String,
}

impl Display for GptResponseChoice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponseUsage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u16,
}
impl GptResponseUsage {
    pub fn add(&mut self, new: &GptResponseUsage) {
        self.prompt_tokens += new.prompt_tokens;
        self.completion_tokens += new.completion_tokens;
        self.total_tokens += new.total_tokens;
    }
}

impl Display for GptResponseUsage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let prompt_t = &self.prompt_tokens.to_string();
        let prompt_c = &self.completion_tokens.to_string();
        let total = &self.total_tokens.to_string();

        let skin = MadSkin::default();
        let text_template = TextTemplate::from(
            r#"
    |:-:|:-:|:-:|
    |**Prompt tokens**|**Response tokens**|**Total**|
    |:-:|:-:|:-:|
    ${module-rows
    |**${prompt}**|**${cmp}**|${total}|
    }
    |-|-|-|
    "#,
        );
        let mut expander = text_template.expander();
        expander
            .sub("module-rows")
            .set("prompt", prompt_t)
            .set("cmp", prompt_c)
            .set("total", total);
        skin.print_expander(expander);
        write!(f, "")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GptResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<GptResponseChoice>,
    pub usage: GptResponseUsage,
    pub system_fingerprint: String,
}

impl Display for GptResponse {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.choices[0])
    }
}

#[derive(Serialize, Debug)]
pub struct GptBody {
    pub model: Option<String>,
    pub messages: Option<Vec<GptMessage>>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub top_p: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
}

pub struct Gpt {
    openai_key: String,
    pub default_model: String,
    pub usage: bool,
    pub append: bool,
}

impl Gpt {
    pub fn new(openai_key: String, args: Cli) -> Self {
        Gpt {
            default_model: String::from("gpt-4o-mini"),
            openai_key,
            usage: false,
            append: args.append,
        }
    }

    pub async fn default_completion(client: &Client, gpt_body: GptBody) -> Result<GptResponse, ()> {
        let response = client
            .as_ref()
            .post(format!("{}/v1/chat/completions", client.server))
            .header(AUTHORIZATION, format!("Bearer {}", client.gpt.openai_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&gpt_body)
            .send()
            .await
            .expect("Request failed");

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<GptResponse>().await {
                Ok(data) => {
                    // Log History
                    file::remove_tmp_file(&client.chat_history_fn);
                    if client.gpt.append && gpt_body.messages.is_some() {
                        let mut hist = gpt_body.messages.unwrap();
                        hist.insert(hist.len() - 1, data.choices[0].message.clone());
                        file::store_tmp_file(&hist, &client.chat_history_fn);
                    } else {
                        let hist: Vec<GptMessage> = vec![data.choices[0].message.clone()];
                        file::store_tmp_file(&hist, &client.chat_history_fn);
                    }
                    // Log usage
                    let history = file::read_tmp_file::<GptResponseUsage>(&client.usage_log_fn);
                    match history {
                        Some(mut hist) => {
                            hist.add(&data.usage);
                            file::remove_tmp_file(&client.usage_log_fn);
                            file::store_tmp_file(&hist, &client.usage_log_fn);
                        }
                        None => {
                            file::store_tmp_file(&data.usage, &client.usage_log_fn);
                        }
                    };

                    if client.gpt.usage {
                        println!("{}", data.usage);
                    }
                    return Ok(data);
                }
                Err(e) => panic!("{}", e.to_string()),
            },
            _other => println!("{:#?}", response.text().await),
        }

        Err(())
    }
}
