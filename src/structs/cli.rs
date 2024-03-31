use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum ComletionModels {
    Gpt3Turbo,
    Gpt4Pre,
    Gpt4,
}

impl ToString for ComletionModels {
    fn to_string(&self) -> String {
        match self {
            ComletionModels::Gpt3Turbo => "gpt-3.5-turbo-0125".to_string(),
            ComletionModels::Gpt4Pre => "gpt-4-0125-preview".to_string(),
            ComletionModels::Gpt4 => "gpt-4".to_string(),
        }
    }
}

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
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    // #[clap(subcommand)]
    ///Default chat gpt
    D(GptCompletionArgs),

    ///Translation pre-promt (default language: en)
    T(GptCompletionArgs),

    ///Correction pre-promt (default language: fr)
    C(GptCompletionArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(propagate_version = false)]
pub struct Cli {
    ///Display the number of tokens used
    #[arg(short, long, default_value_t = false)]
    pub usage: bool,

    #[command(subcommand)]
    pub cmd: Command,
}
