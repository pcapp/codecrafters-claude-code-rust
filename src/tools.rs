use serde::Deserialize;
use serde_json::{Value, json};

use crate::wire::ToolCall;

const READ: &str = "Read";

fn read_spec() -> Value {
    json!({"type": "function",
    "function": {
      "name": READ,
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
    }})
}

pub fn specs() -> Value {
    json!(read_spec())
}

#[derive(Deserialize)]
struct ReadArgs {
    file_path: String,
}

fn read_file_tool(arguments: &str) -> serde_json::Value {
    let args: ReadArgs = match serde_json::from_str(arguments) {
        Ok(args) => args,
        Err(err) => {
            return serde_json::json!({
                "ok": false,
                "error": format!("Invalid Read arguments: {}", err)
            });
        }
    };

    match std::fs::read_to_string(args.file_path) {
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

pub fn execute_tool_call(call: &ToolCall) -> serde_json::Value {
    match call.function.name.as_str() {
        READ => read_file_tool(&call.function.arguments),

        unimplemented_tool => serde_json::json!({
          "ok": false,
          "error": format!("Unimplemented tool: {}", unimplemented_tool)
        }),
    }
}
