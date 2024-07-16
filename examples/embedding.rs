use openai_rst::{
    client::Client,
    embedding::EmbeddingRequest,
    models::{EmbeddingsModels, Model},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env().unwrap();

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
