use openai_rst::{
    chat_completion::{
        ChatCompletionMessage, ChatCompletionRequest, Content, FinishReason, Function,
        FunctionParameters, JSONSchemaDefine, JSONSchemaType, Tool, ToolChoiceType, ToolType,
    },
    client::Client,
    common::MessageRole,
    models::{Model, GPT3},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, vec};

fn get_coin_price(coin: &str) -> f64 {
    let coin = coin.to_lowercase();
    match coin.as_str() {
        "btc" | "bitcoin" => 10000.0,
        "eth" | "ethereum" => 1000.0,
        _ => 0.0,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut properties = HashMap::new();
    properties.insert(
        "coin".to_string(),
        Box::new(JSONSchemaDefine {
            schema_type: Some(JSONSchemaType::String),
            description: Some("The cryptocurrency to get the price of".to_string()),
            ..Default::default()
        }),
    );

    let req = ChatCompletionRequest::new(
        Model::GPT3(GPT3::GPT35Turbo),
        vec![ChatCompletionMessage {
            role: MessageRole::User,
            content: Content::Text(String::from("What is the price of Ethereum?")),
            name: None,
        }],
    )
    .tools(vec![Tool {
        r#type: ToolType::Function,
        function: Function {
            name: String::from("get_coin_price"),
            description: Some(String::from("Get the price of a cryptocurrency")),
            parameters: FunctionParameters {
                schema_type: JSONSchemaType::Object,
                properties: Some(properties),
                required: Some(vec![String::from("coin")]),
            },
        },
    }])
    .tool_choice(ToolChoiceType::Auto);

    let result = client.chat_completion(req)?;

    match result.choices[0].finish_reason {
        None => {
            println!("No finish_reason");
            println!("{:?}", result.choices[0].message.content);
        }
        Some(FinishReason::stop) => {
            println!("Stop");
            println!("{:?}", result.choices[0].message.content);
        }
        Some(FinishReason::length) => {
            println!("Length");
        }
        Some(FinishReason::tool_calls) => {
            println!("ToolCalls");
            #[derive(Deserialize, Serialize)]
            struct Currency {
                coin: String,
            }
            let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
            for tool_call in tool_calls {
                let name = tool_call.function.name.clone().unwrap();
                let arguments = tool_call.function.arguments.clone().unwrap();
                let c: Currency = serde_json::from_str(&arguments)?;
                let coin = c.coin;
                if name == "get_coin_price" {
                    let price = get_coin_price(&coin);
                    println!("{} price: {}", coin, price);
                }
            }
        }
        Some(FinishReason::content_filter) => {
            println!("ContentFilter");
        }
        Some(FinishReason::null) => {
            println!("Null");
        }
    }
    Ok(())
}
