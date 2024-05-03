use openai_rst::{
    client::Client,
    embedding::EmbeddingRequest,
    models::{EmbeddingsModels, Model},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut req = EmbeddingRequest::new(
        Model::Embedding(EmbeddingsModels::TextEmbeddingAda002),
        "story time".to_string(),
    );
    req.dimensions = Some(10);

    let result = client.embedding(req).await?;
    println!("{:?}", result.data);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example embedding
