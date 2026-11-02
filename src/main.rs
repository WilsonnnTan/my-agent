use clap::Parser;
use serde_json::{Value, json};
use my_agent::{
  config::Config,
  tools::ToolCall,
  openai::client::LlmClient,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
  #[arg(short = 'p', long)]
  prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  let config = Config::load();

  let llm_client = LlmClient::new(&config);

  let mut messages: Vec<Value> = Vec::new();

  messages.push(json!(
    {
      "role": "user",
      "content": args.prompt,
    }
  ));

  let response: Value = llm_client.chat(&messages).await?;

  let data: serde_json::Value = serde_json::from_value(response)?;
  // println!("{:#?}", data);

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
