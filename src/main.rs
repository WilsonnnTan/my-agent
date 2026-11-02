use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};
use serde_json;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
  #[arg(short = 'p', long)]
  prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

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
      "model": "nvidia/nemotron-3-ultra-550b-a55b:free",
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
      ],
    }))
    .await?;

  let data: serde_json::Value = serde_json::from_value(response)?;
  println!("{:#?}", data);

  if let Some(tool_calls) = data
                            .pointer("/choices/0/message/tool_calls")
                            .and_then(|tool_calls| tool_calls.as_array())
                            .filter(|arr| !arr.is_empty())
  {
    for tool_call_field in tool_calls {
      let tool_type = tool_call_field
                        .pointer("/type")
                        .and_then(|name| name.as_str())
                        .ok_or_else(|| "tool_call missing type")?;

      let name = tool_call_field
                  .pointer("/function/name")
                  .and_then(|name| name.as_str())
                  .ok_or_else(|| "tool_call missing name")?;

      let arguments = tool_call_field
                  .pointer("/function/arguments")
                  .and_then(|arguments| arguments.as_str())
                  .ok_or_else(|| "tool_call missing arguments")?;
            
      use my_agent::tools::ToolCall;
      let tool_call = ToolCall::new(tool_type, name, arguments);
      match tool_call.call() {
        Ok(output) => { 
          // this println can be deleted later after agent loop is implemented
          println!("{}", output);
        },
        Err(e) => eprintln!("failed to call a tool: {}", e),
      }
    }
  }

  // TODO: Uncomment the lines below to pass the first stage
  if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
    println!("{}", content);
  }

  Ok(())
}
