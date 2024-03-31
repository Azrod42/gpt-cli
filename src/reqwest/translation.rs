use core::panic;

use crate::structs::{
    cli::GptCompletionArgs,
    gpt::{Gpt, GptBody, GptMessage},
};

use super::Client;

impl Client {
    pub async fn translate(&self, args: GptCompletionArgs) -> Result<(), ()> {
        if args.prompt.len() < 1 {
            return Ok(());
        }

        let gpt_message = GptMessage {
            role: String::from("user"),
            content: format!(
                "Translate this sentence in {}: {}",
                args.language.map_or(String::from("english"), |x| x),
                args.prompt.join(" ")
            ),
        };

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
            messages: Some(vec![gpt_message]),
        };

        let res = Gpt::default_completion(self, gpt_body).await;
        match res {
            Ok(data) => print!("{}", data),
            Err(_) => panic!("Fail"),
        }
        Ok(())
    }
}
