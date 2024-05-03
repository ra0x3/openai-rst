use openai_rst::{
    assistant::AssistantRequest,
    client::Client,
    common::MessageRole,
    message::CreateMessageRequest,
    models::{Model, GPT4},
    run::CreateRunRequest,
    thread::CreateThreadRequest,
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut tools = HashMap::new();
    tools.insert("type".to_string(), "code_interpreter".to_string());

    let req = AssistantRequest::new(Model::GPT4(GPT4::GPT40125Preview));
    let req = req
        .clone()
        .description("this is a test assistant".to_string());
    let req = req.clone().instructions("You are a personal math tutor. When asked a question, write and run Python code to answer the question.".to_string());
    let req = req.clone().tools(vec![tools]);
    println!("{:?}", req);

    let result = client.create_assistant(req).await?;
    println!("{:?}", result.id);

    let thread_req = CreateThreadRequest::new();
    let thread_result = client.create_thread(thread_req).await?;
    println!("{:?}", thread_result.id.clone());

    let message_req = CreateMessageRequest::new(
        MessageRole::User,
        "`I need to solve the equation 3x + 11 = 14. Can you help me?".to_string(),
    );

    let message_result = client
        .create_message(thread_result.id.clone(), message_req)
        .await?;
    println!("{:?}", message_result.id.clone());

    let run_req = CreateRunRequest::new(result.id);
    let run_result = client.create_run(thread_result.id.clone(), run_req).await?;

    loop {
        let run_result = client
            .retrieve_run(thread_result.id.clone(), run_result.id.clone())
            .await?;
        if run_result.status == "completed" {
            break;
        } else {
            println!("waiting...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    let list_message_result = client.list_messages(thread_result.id.clone()).await?;
    for data in list_message_result.data {
        for content in data.content {
            println!(
                "{:?}: {:?} {:?}",
                data.role, content.text.value, content.text.annotations
            );
        }
    }

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-rst --example assistant
