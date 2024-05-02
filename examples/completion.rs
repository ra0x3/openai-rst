use crate::models;
use openai_rst::api::Client;
use openai_rst::completion::{self, CompletionRequest};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = CompletionRequest::new(
        models::GPT3::Davinci.into(),
        String::from("What is Bitcoin?"),
    )
    .max_tokens(3000)
    .temperature(0.9)
    .top_p(1.0)
    .stop(vec![String::from(" Human:"), String::from(" AI:")])
    .presence_penalty(0.6)
    .frequency_penalty(0.0);

    let result = client.completion(req)?;
    println!("{:}", result.choices[0].text);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example completion
