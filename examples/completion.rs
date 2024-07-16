use openai_rst::{
    client::Client,
    completion::CompletionRequest,
    models::{Model, GPT3},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env().unwrap();

    let req = CompletionRequest::new(
        Model::GPT3(GPT3::GPT35Turbo),
        String::from("What is Bitcoin?"),
    )
    .max_tokens(3000)
    .temperature(0.9)
    .top_p(1.0)
    .stop(vec![String::from(" Human:"), String::from(" AI:")])
    .presence_penalty(0.6)
    .frequency_penalty(0.0);

    let result = client.completion(req).await?;
    println!("{:}", result.choices[0].text);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example completion
