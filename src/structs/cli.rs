use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum ComletionModels {
    Gpt4o,
    Gpt4oMini,
    O1Mini,
    O1Preview,
}

impl ToString for ComletionModels {
    fn to_string(&self) -> String {
        match self {
            ComletionModels::Gpt4o => "gpt-4o".to_string(),
            ComletionModels::Gpt4oMini => "gpt-4o-mini".to_string(),
            ComletionModels::O1Preview => "o1-preview".to_string(),
            ComletionModels::O1Mini => "o1-mini".to_string(),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct UsageArgs {}

#[derive(Args, Debug, Clone)]
pub struct GptCompletionArgs {
    // #[clap(raw = true)]
    pub prompt: Vec<String>,

    ///You can use any language you want
    #[clap(short, long)]
    pub language: Option<String>,

    #[clap(value_enum, short, long)]
    pub model: Option<ComletionModels>,

    ///Maximum numbers of tokens
    #[clap(short, long, default_value_t = 750, value_name = "0 - 16000")]
    pub tokens_max: i32,

    #[clap(long, value_name = "0 - 2.0")]
    pub temperature: Option<f32>,

    #[clap(short, long, value_name = "0 - 2.0")]
    pub frequency_penalty: Option<f32>,

    #[clap(short, long, value_name = "0 - 2.0")]
    pub presence_penalty: Option<f32>,

    #[clap(long, value_name = "0 - 1.0")]
    pub top_p: Option<f32>,

    ///Continue conversation with previous message
    #[clap(short, long, default_value_t = false)]
    pub append: bool,

    //Display the usage in token of this request.
    #[clap(short, long, default_value_t = false)]
    pub usage: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    // #[clap(subcommand)]
    ///Default chat gpt
    D(GptCompletionArgs),

    ///Translation pre-promt (default language: en)
    T(GptCompletionArgs),

    ///Correction pre-promt (default language: en)
    C(GptCompletionArgs),

    ///Display the total usage of tokens.
    Usage(UsageArgs),
}

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
#[clap(propagate_version = false)]
pub struct Cli {
    //Continue the previous conversation
    #[arg(short, long, default_value_t = false)]
    pub append: bool,

    #[command(subcommand)]
    pub cmd: Command,
}
