use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

fn read_file_tool(file_path: &str) -> serde_json::Value {
    match std::fs::read_to_string(file_path) {
        Ok(contents) => serde_json::json!({
          "ok": true,
          "content": contents
        }),
        Err(err) => serde_json::json!({
          "ok": false,
          "error": err.to_string()
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": "anthropic/claude-haiku-4.5",
            "tools": [
              {
                "type": "function",
                "function": {
                  "name": "Read",
                  "description": "Read and return the contents of a file",
                  "parameters": {
                    "type": "object",
                    "properties": {
                      "file_path": {
                        "type": "string",
                        "description": "The path to the file to read"
                      }
                    },
                    "required": ["file_path"]
                  }
                }
              }
            ]
        }))
        .await?;

    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        for call in tool_calls {
            let name = call["function"]["name"].as_str().unwrap();

            // println!("Use {}", name);

            if let Some(arguments_str) = call["function"]["arguments"].as_str() {
                let arguments: Value = serde_json::from_str(arguments_str)?;

                if let Some(arguments_obj) = arguments.as_object() {
                    if let Some(file_path) = arguments_obj["file_path"].as_str() {
                        let result = read_file_tool(file_path);
                        if result["ok"].as_bool() == Some(true) {
                            if let Some(content) = result["content"].as_str() {
                                println!("{}", content);
                            } else if let Some(error) = result["error"].as_str() {
                                eprintln!("Error: {}", error);
                            }
                        }
                    }
                } else {
                    println!("\targuments: {}", arguments);
                }
            }
        }
    } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}
