use aws_sdk_bedrockruntime::types::ContentBlock;
use aws_sdk_bedrockruntime::types::GuardrailStreamConfiguration;
use aws_sdk_bedrockruntime::types::InferenceConfiguration;
use aws_sdk_bedrockruntime::types::Message;
use aws_sdk_bedrockruntime::types::ConversationRole;
use aws_sdk_bedrockruntime::types::SystemContentBlock;

use std::io::{self, Write};
use colored::Colorize;

use dotenvy::dotenv;
use std::env;

// Some helper functions in lib.rs
use hello_bedrock::BedrockConverseStreamError;
use hello_bedrock::get_converse_stream_output_text;
use hello_bedrock::print_header;

// NOTE: PLEASE UNCOMMENT WHICH MODEL YOU WOULD LIKE TO USE:

// const MODEL_ID: &str = "us.amazon.nova-pro-v1:0";
// const MODEL_ID: &str = "us.anthropic.claude-3-7-sonnet-20250219-v1:0";
// const MODEL_ID: &str = "anthropic.claude-3-5-sonnet-20241022-v2:0";
const MODEL_ID: &str = "us.meta.llama3-3-70b-instruct-v1:0";


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = hello_bedrock::configure_aws("us-east-1".into(),None).await;
    // Bedrock Runtime
    let bedrockruntime = aws_sdk_bedrockruntime::Client::new(&config);

    // Load Environment variables
    dotenv()?;
    let guardrail_id = env::var("GUARDRAIL_ID")?;

    print_header("Guardrails Example");
    println!("----------------------------------------");
    println!("Using Model: {}", MODEL_ID.yellow());

    // Get user Input
    print!("Prompt: ");
    io::stdout().flush().unwrap();

    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).expect("Failed to read line");

    let system_message = SystemContentBlock::Text(String::from("You are a helpful assistant."));

    let inference_config = InferenceConfiguration::builder()
        .top_p(0.8)
        .temperature(0.5)
        .max_tokens(2048)
        .build();

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(prompt.clone()))
        .build()?;

    let guardrails = GuardrailStreamConfiguration::builder()
        .guardrail_identifier(guardrail_id)
        .guardrail_version("2")
        .build()?;

    let response = bedrockruntime.converse_stream()
        .messages(message)
        .model_id(MODEL_ID)
        .inference_config(inference_config)
        .system(system_message)
        .guardrail_config(guardrails)
        .send()
        .await; 
    

    let mut stream = match response {
        Ok(output) => Ok(output.stream),
        Err(e) => Err(BedrockConverseStreamError::from(
            e.as_service_error().unwrap(),
        )),
    }?;


    loop {
        let token = stream.recv().await;
        match token {
            Ok(Some(text)) => {
                let next = get_converse_stream_output_text(text)?;
                print!("{}", next);
                Ok(())
            }
            Ok(None) => break,
            Err(e) => Err(e
                .as_service_error()
                .map(BedrockConverseStreamError::from)
                .unwrap_or(BedrockConverseStreamError(
                    "Unknown error receiving stream".into(),
                ))),
        }?
    }

    println!();

    Ok(())
}
