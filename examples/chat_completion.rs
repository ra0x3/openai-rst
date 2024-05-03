use openai_rst::{
    chat_completion::{ChatCompletionMessage, ChatCompletionRequest, Content},
    client::Client,
    common::MessageRole,
    models::{Model, GPT4},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(
        Model::GPT4(GPT4::GPT4),
        vec![ChatCompletionMessage {
            role: MessageRole::User,
            content: Content::Text(String::from("What is bitcoin?")),
            name: None,
        }],
    );

    let result = client.chat_completion(req)?.await;
    println!("Content: {:?}", result.message_content());
    println!("Response Headers: {:?}", result.headers);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example chat_completion
