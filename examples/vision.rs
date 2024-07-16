use openai_rst::{
    chat_completion::{self, ChatCompletionRequest},
    client::Client,
    common::MessageRole,
    models::{Model, GPT4},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()).unwrap();

    let req = ChatCompletionRequest::new(
        Model::GPT4(GPT4::GPT40125Preview),
        vec![chat_completion::ChatCompletionMessage {
            role: MessageRole::User,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from("What’s in this image?")),
                    image_url: None,
                },
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::image_url,
                    text: None,
                    image_url: Some(chat_completion::ImageUrlType {
                        url: String::from(
                            "https://upload.wikimedia.org/wikipedia/commons/5/50/Bitcoin.png",
                        ),
                    }),
                },
            ]),
            name: None,
        }],
    );

    let result = client.chat_completion(req).await?;
    println!("{:?}", result.choices[0].message.content);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example vision
