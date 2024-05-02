use openai_rst::api::Client;
use openai_rst::audio::{self, AudioSpeechRequest, TTS_1};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = AudioSpeechRequest::new(
        TTS_1.to_string(),
        String::from("Money is not problem, Problem is no money"),
        audio::VOICE_ALLOY.to_string(),
        String::from("problem.mp3"),
    );

    let result = client.audio_speech(req)?;
    println!("{:?}", result);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example text_to_speech
