mod tools;
mod wire;

use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};
use wire::ChatResponse;

use crate::tools::{execute_tool_call, specs};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let args = Args::parse();

    dotenvy::dotenv().ok();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    let mut messages: Vec<Value> = vec![json!({
        "role": "user",
        "content": args.prompt,
    })];

    let tools = json!([specs()]);

    const MAX_LOOPS: u8 = 10;

    for _ in 0..MAX_LOOPS {
        let request = json!({
            "messages": messages,
            "model": "anthropic/claude-haiku-4.5",
            "tools": tools,
        });

        tracing::debug!(
          event = "llm_request",
          n_messages = messages.len(),
          payload = %request,
        );

        let response: Value = client.chat().create_byot(request).await?;

        let raw_message = response["choices"][0]["message"].clone();

        let response = match serde_json::from_value::<ChatResponse>(response) {
            Ok(parsed_response) => parsed_response,
            Err(err) => {
                eprintln!("Received a malformed OpenRouter response.");
                eprintln!("Error: {}", err);
                return Ok(());
            }
        };

        let Some(choice) = response.choices.first() else {
            eprintln!("No choices returned!");
            return Ok(());
        };

        let message = &choice.message;

        messages.push(raw_message);

        let tool_calls = message.tool_calls.as_deref().unwrap_or_default();

        if tool_calls.is_empty() {
            if let Some(content) = &message.content {
                println!("{}", content);
            }
            return Ok(());
        }

        for tool_call in tool_calls {
            let result = execute_tool_call(tool_call);

            if let Some(error) = result["error"].as_str() {
                eprintln!("Tool call error: {}", error);
            }

            messages.push(json!({
              "role": "tool",
              "tool_call_id": tool_call.id,
              "content": result.to_string()
            }));
        }
    }

    eprintln!(
        "The agentic loop exceeded the max iterations ({}).",
        MAX_LOOPS,
    );
    Ok(())
}
