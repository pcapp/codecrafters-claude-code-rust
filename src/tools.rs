use serde::Deserialize;
use serde_json::{Value, json};

use crate::wire::ToolCall;

const READ: &str = "Read";
const WRITE: &str = "Write";

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

fn write_spec() -> Value {
    json!({
  "type": "function",
  "function": {
    "name": WRITE,
    "description": "Write content to a file",
    "parameters": {
      "type": "object",
      "required": ["file_path", "content"],
      "properties": {
        "file_path": {
          "type": "string",
          "description": "The path of the file to write to"
        },
        "content": {
          "type": "string",
          "description": "The content to write to the file"
        }
      }
    }
  }
})
}

pub fn specs() -> Value {
    json!([read_spec(), write_spec()])
}

#[derive(Deserialize)]
struct ReadArgs {
    file_path: String,
}

#[derive(Deserialize)]
struct WriteArgs {
    file_path: String,
    content: String
}

fn read_file_tool(arguments: &str) -> Value {
    let args: ReadArgs = match serde_json::from_str(arguments) {
        Ok(args) => args,
        Err(err) => {
            return json!({
                "ok": false,
                "error": format!("Invalid Read arguments: {}", err)
            });
        }
    };

    match std::fs::read_to_string(args.file_path) {
        Ok(contents) => json!({
          "ok": true,
          "content": contents
        }),
        Err(err) => json!({
          "ok": false,
          "error": err.to_string()
        }),
    }
}

fn write_file_tool(arguments: &str) -> Value {
    let args: WriteArgs = match serde_json::from_str(arguments) {
        Ok(args) => args,
        Err(err) => {
            return json!({
                "ok": false,
                "error": format!("Invalid Write arguments: {}", err)
            });
        }
    };

    match std::fs::write(args.file_path, args.content) {
        Ok(contents) => json!({
          "ok": true,
          "content": contents
        }),
        Err(err) => json!({
          "ok": false,
          "error": err.to_string()
        }),
    }
}

pub fn execute_tool_call(call: &ToolCall) -> Value {
    match call.function.name.as_str() {
        READ => read_file_tool(&call.function.arguments),
        WRITE => write_file_tool(&call.function.arguments),

        unimplemented_tool => json!({
          "ok": false,
          "error": format!("Unimplemented tool: {}", unimplemented_tool)
        }),
    }
}
