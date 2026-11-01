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
    for tool_call in tool_calls {
      let name = tool_call
                  .pointer("/function/name")
                  .and_then(|name| name.as_str())
                  .ok_or_else(|| "tool_call missing name")?;

      let arguments = tool_call
                  .pointer("/function/arguments")
                  .and_then(|arguments| arguments.as_str())
                  .ok_or_else(|| "tool_call missing arguments")?;
            
      let args: Value = serde_json::from_str(arguments)?;

      let file_path = args
                  .pointer("/file_path")
                  .and_then(|path| path.as_str())
                  .ok_or_else(|| "arguments missing file_path")?;

      use my_agent::tools::read_file::ReadFile;
      use my_agent::tools::Run;
      ReadFile::run(file_path);
    }
  }

  // You can use print statements as follows for debugging, they'll be visible when running tests.
  eprintln!("Logs from your program will appear here!");

  // TODO: Uncomment the lines below to pass the first stage
  if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
    println!("{}", content);
  }

  Ok(())
}
