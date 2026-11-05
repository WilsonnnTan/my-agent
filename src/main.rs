use clap::Parser;
use serde_json::{Value, json};
use my_agent::{
  config::Config,
  openai::client::LlmClient,
  skill::{skills_advertisement, load_skills},
  tools::response_tool_calls_parser,
  prompt::parse_prompt,
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

  // Load skills
  let skills = load_skills()?;
  let skills_ad = skills_advertisement(&skills);

  let prompt = parse_prompt(&args.prompt, &skills);

  // Track messages session
  let mut messages: Vec<Value> = Vec::new();
  messages.push(json!(
    {
      "role": "system",
      "content": skills_ad,
    }
  ));
  messages.push(json!(
    {
      "role": "user",
      "content": prompt,
    }
  ));

  loop {
    let response: Value = llm_client.chat(&messages).await?;

    // push AI response to messages to keep track of conversation
    if let Some(ai_message) = response.pointer("/choices/0/message") {
      messages.push(ai_message.clone());
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
      let mut tool_message: Value = json!(
        {
          "role": "tool",
        }
      );

      match tool_call.call() {
        Ok(output) => {
          tool_message["content"] = output.into();
        },
        Err(e) => {
          let error_message = format!("failed to call {} tool: {}", tool_call.function.name, e);
          eprintln!("{}", error_message);
          tool_message["content"] = error_message.into();
        },
      }

      tool_message["tool_call_id"] = tool_call.id.into();
      messages.push(tool_message);
    }
  }

  // UNCOMMENT: for debug
  // println!("{:#?}", messages);

  Ok(())
}
