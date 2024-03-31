use crate::structs::cli::Cli;
use clap::Parser;
use reqwest::Client;
use structs::cli;

pub mod reqwest;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let openai_key = std::env::var("OPENAI_KEY").expect("OPENAI_KEY");
    let cli = Cli::parse();
    let client = Client::new(openai_key, cli.usage);

    match cli.cmd {
        cli::Command::T(args) => client.translate(args).await?,
        cli::Command::C(args) => client.correct(args).await?,
        cli::Command::D(args) => client.default_completion(args).await?,
    }

    Ok(())
}
