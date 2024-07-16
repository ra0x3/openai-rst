use openai_rst::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env().unwrap();
    let req = "What is bitcoin?".into();
    let result = client.chat_completion(req).await?;

    println!("Content: {:?}", result.get_choice());
    println!("Response Headers: {:?}", result.headers);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example chat_completion
