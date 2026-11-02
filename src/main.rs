use clap::Parser;
use serde_json::{Value, json};
use my_agent::{
  config::Config,
  openai::client::LlmClient,
  openai::response_tool_calls_parser
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

  loop {
    let response: Value = llm_client.chat(&messages).await?;

    // push AI response to messages to keep track of conversation
    if let Some(message) = response.pointer("/choices/0/message") {
      messages.push(message.clone());
    }

    let tool_calls = response_tool_calls_parser(&response)?;

    // break agent loop if `tool_calls` is empty or `finish_reason` is "stop"
    if tool_calls.is_empty() || response["choices"][0]["finish_reason"].as_str() == Some("stop") {
      if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
      }
      break;
    }

    for tool_call in tool_calls {
      match tool_call.call() {
        Ok(output) => {
          messages.push(json!(
            {
              "role": "tool",
              "tool_call_id": tool_call.id,
              "content": output,
            }
          ));
          println!("{}", output);
        },
        Err(e) => eprintln!("failed to call a tool: {}", e),
      }
    }
  }

  Ok(())
}
