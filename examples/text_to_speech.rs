use openai_rst::audio::{self, AudioSpeechRequest, TTS_1};
use openai_rst::client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string()).unwrap();

    let req = AudioSpeechRequest::new(
        TTS_1.to_string(),
        String::from("Money is not problem, Problem is no money"),
        audio::VOICE_ALLOY.to_string(),
        String::from("problem.mp3"),
    );

    let result = client.audio_speech(req).await?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example text_to_speech
