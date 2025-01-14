⚠️ Forked from [openai-rs-api](https://github.com/dongri/openai-rst).

# OpenAI API client library for Rust (unofficial)
The OpenAI API client Rust library provides convenient access to the OpenAI API from Rust applications.

Check out the [docs.rs](https://docs.rs/openai-rst).

## Installation:
Cargo.toml
```toml
[dependencies]
openai-rst = "0.1.0"
```

## Usage
The library needs to be configured with your account's secret key, which is available on the [website](https://platform.openai.com/account/api-keys). We recommend setting it as an environment variable. Here's an example of initializing the library with the API key loaded from an environment variable and creating a completion:

### Set OPENAI_API_KEY to environment variable
```bash
$ export OPENAI_API_KEY=sk-xxxxxxx
```

### Create client
```rust
let client = Client::from_env().unwrap();
```

### Create request
```rust
// Single request
let req = ChatCompletionRequest::new(
    Model::GPT4(GPT4::GPT4),
    ChatCompletionMessage {
        role: MessageRole::User,
        content: Content::Text(String::from("What is bitcoin?")),
        name: None,
    },
);

// Multiple requests
let req = ChatCompletionRequest::new_multi(
    Model::GPT4(GPT4::GPT4),
    vec![ChatCompletionMessage {
        role: MessageRole::User,
        content: Content::Text(String::from("What is bitcoin?")),
        name: None,
    }],
);
```

### Send request
```rust
let result = client.chat_completion(req)?;
println!("Content: {:?}", result.get_choice());
```

### Set OPENAI_API_BASE to environment variable (optional)
```bash
$ export OPENAI_API_BASE=https://api.openai.com/v1
```

## Example of chat completion
```rust
use openai_rst::{chat_completion::ChatCompletionRequest, client::Client, models::{Model, GPT4}};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::from_env().unwrap();
    let req = "What is bitcoin?".into();
    let result = client.chat_completion(req).await?;
    
    println!("Content: {:?}", result.get_choice());
    println!("Response Headers: {:?}", result.headers);

    Ok(())
}
```
More Examples: [examples](https://github.com/dongri/openai-rst/tree/main/examples)

Check out the [full API documentation](https://platform.openai.com/docs/api-reference/completions) for examples of all the available functions.

## Supported APIs
- [x] [completions](https://platform.openai.com/docs/api-reference/completions)
- [x] [Chat](https://platform.openai.com/docs/api-reference/chat)
- [x] [Edits](https://platform.openai.com/docs/api-reference/edits)
- [x] [Images](https://platform.openai.com/docs/api-reference/images)
- [x] [Embeddings](https://platform.openai.com/docs/api-reference/embeddings)
- [x] [Audio](https://platform.openai.com/docs/api-reference/audio)
- [x] [Files](https://platform.openai.com/docs/api-reference/files)
- [x] [Fine-tuning](https://platform.openai.com/docs/api-reference/fine-tuning)
- [x] [Moderations](https://platform.openai.com/docs/api-reference/moderations)
- [x] [Function calling](https://platform.openai.com/docs/guides/gpt/function-calling)
- [x] [Assistants](https://platform.openai.com/docs/assistants/overview)

## License
This project is licensed under [MIT license](https://github.com/dongri/openai-rst/blob/main/LICENSE).
