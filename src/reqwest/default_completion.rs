use core::panic;

use crate::{
    file,
    structs::{
        cli::GptCompletionArgs,
        gpt::{Gpt, GptBody, GptMessage},
    },
};

use super::Client;

impl Client {
    pub async fn default_completion(
        &mut self,
        args: GptCompletionArgs,
        pre_prompt_raw: Option<&str>,
    ) -> Result<(), ()> {
        if args.prompt.len() < 1 {
            return Ok(());
        }

        let mut pre_prompt = String::new();

        if pre_prompt_raw.is_some() {
            pre_prompt = format!(
                "{} {}: ",
                pre_prompt_raw.unwrap(),
                args.language.map_or(String::from("english"), |x| x),
            )
        }

        let mut gpt_messages = vec![GptMessage {
            role: String::from("user"),
            content: format!("{}{}", pre_prompt, args.prompt.join(" ")),
        }];

        if self.gpt.append || args.append {
            self.gpt.append = true;
            let history = file::read_tmp_file::<Vec<GptMessage>>(&self.chat_history_fn);
            match history {
                Some(data) => {
                    for item in data {
                        gpt_messages.insert(gpt_messages.len() - 1, item)
                    }
                }
                None => {}
            };
        }

        if args.usage {
            self.gpt.usage = true;
        }

        let gpt_body = GptBody {
            model: Some(
                args.model
                    .map_or(self.gpt.default_model.clone(), |x| x.to_string()),
            ),
            frequency_penalty: args.frequency_penalty,
            temperature: args.temperature,
            top_p: args.top_p,
            max_tokens: Some(args.tokens_max),
            presence_penalty: args.presence_penalty,
            messages: Some(gpt_messages),
        };

        let res = Gpt::default_completion(self, gpt_body).await;
        match res {
            Ok(data) => print!("{}", data),
            Err(_) => panic!("Request failed"),
        }
        Ok(())
    }
}
