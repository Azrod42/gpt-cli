use crate::structs::cli::Cli;
use clap::Parser;
use reqwest::Client;
use structs::cli;

pub mod file;
pub mod reqwest;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let openai_key = std::env::var("OPENAI_KEY").expect("OPENAI_KEY");
    let cli = Cli::parse();
    let mut client = Client::new(openai_key, cli.clone());

    match cli.cmd {
        cli::Command::T(args) => {
            client
                .default_completion(args, Some("Translate this sentence in"))
                .await?
        }
        cli::Command::C(args) => {
            client
                .default_completion(args, Some("Correct this sentence in"))
                .await?
        }
        cli::Command::D(args) => client.default_completion(args, None).await?,
        cli::Command::Usage(_) => client.print_usage(),
    }

    Ok(())
}
