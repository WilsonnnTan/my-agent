use clap::Parser;
use serde_json::{Value, json};
use my_agent::{
  agent::Agent, config::Config, lsp_client::{self, LspClient}, openai::client::LlmClient, prompt::parse_prompt, skill::{skill_loader::SkillLoader, skills_advertisement},
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

  let lsp = LspClient::from_environment()?;

  // connect to LSP Client
  match lsp {
    Some(_) => eprintln!("LSP client connected"),
    None => eprintln!("LSP client failed to connect"),
  };

  // load config and llm_client using OnceLock so it is only initialized once
  Config::init();
  LlmClient::init();

  // Load skills using OnceLock so it is only initialized once
  let skills = SkillLoader::init()?;
  let skills_ad = skills_advertisement(skills);

  let prompt = parse_prompt(args.prompt, skills);

  // dbg!(&prompt);

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

  // Run agent loop
  match Agent::run_loop(&mut messages).await {
    Ok(res) => println!("{}", res),
    Err(e) => {
      eprintln!("Agent failed to run: {}", e);
      std::process::exit(1);
    }
  }

  // dbg!(messages);

  Ok(())
}
