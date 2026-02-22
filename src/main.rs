use rig::agent::stream_to_stdout;
use rig::client::Nothing;
use rig::prelude::*;
use rig::providers::ollama;
use rig::streaming::StreamingPrompt;
use serde::Deserialize;
use std::fs;
use std::time::Instant;

/// This example requires that you have the [`ollama`](https://ollama.com) server running locally.

#[derive(Debug, Deserialize)]
struct Config {
    base_url: String,
    model: String,
}

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Client error: {0}")]
    Client(#[from] rig::http_client::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config error: {0}")]
    Config(#[from] toml::de::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load configuration
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    // Create ollama client
    //
    // In the case of ollama, no API key is necessary, so we can use the `Nothing` struct in its
    // place
    let client: ollama::Client = ollama::Client::builder()
        .api_key(Nothing)
        .base_url(&config.base_url)
        .build()?;

    // Create agent with a single context prompt
    let inn_keeper_agent = client
        .agent(&config.model)
        .preamble("You are an inn keeper in a small town. You are 68 years old and are a no-nonsense kind of guy.")
        .build();

    // Stream the response and print chunks as they arrive
    let start = Instant::now();
    let mut stream = inn_keeper_agent.stream_prompt("I'd like to rent a room.").await;
    stream_to_stdout(&mut stream).await?;
    let duration = start.elapsed();

    println!("\n\nTotal time: {:.2?}", duration);

    Ok(())
}
