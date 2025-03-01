use aws_config::Region;
use aws_sdk_bedrockagentruntime::types::KnowledgeBaseRetrieveAndGenerateConfiguration;
use aws_sdk_bedrockagentruntime::types::RetrieveAndGenerateConfiguration;
use aws_sdk_bedrockagentruntime::types::RetrieveAndGenerateInput;
use hello_bedrock::generate_arn;

use colored::Colorize;
use std::io::{self, Write};

use dotenv::dotenv;
use std::env;

// Some helper functions in lib.rs
use hello_bedrock::BedrockConverseError;
use hello_bedrock::get_account_id;
use hello_bedrock::get_rag_output_text;
use hello_bedrock::print_header;

// NOTE: PLEASE UNCOMMENT WHICH MODEL YOU WOULD LIKE TO USE:

// const MODEL_ID: &str = "us.amazon.nova-pro-v1:0";
// const MODEL_ID: &str = "us.anthropic.claude-3-7-sonnet-20250219-v1:0";
const MODEL_ID: &str = "anthropic.claude-3-5-sonnet-20241022-v2:0";
// const MODEL_ID: &str = "us.meta.llama3-3-70b-instruct-v1:0";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = hello_bedrock::configure_aws("us-east-1".into(), None).await;
    // Bedrock Runtime

    // Bedrock Agent Runtime - used with Knowledge bases
    let agentruntime = aws_sdk_bedrockagentruntime::Client::new(&config);

    // Load Environment variables
    dotenv()?;
    let kb_id = env::var("KB_ID")?;

    // Extract account details so we can generate ARNs
    let region = &config
        .region()
        .cloned()
        .unwrap_or_else(|| Region::from_static("us-east-1"));
    let account_id = get_account_id(&config).await?;
    let model_arn = generate_arn(region.as_ref(), &account_id, MODEL_ID)?;

    print_header("Knowledgebases ");
    println!("----------------------------------------");
    println!("Using Model: {}", MODEL_ID.yellow());

    println!("DEBUG: {:?}", &model_arn);

    // Get user Input
    print!("Prompt: ");
    io::stdout().flush().unwrap();

    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .expect("Failed to read line");

    let rag_input = RetrieveAndGenerateInput::builder()
        .text(String::from(&prompt))
        .build()?;

    let kb_config = KnowledgeBaseRetrieveAndGenerateConfiguration::builder()
        .knowledge_base_id(kb_id)
        .model_arn(model_arn)
        .build()?;

    let rag_config = RetrieveAndGenerateConfiguration::builder()
        .r#type(aws_sdk_bedrockagentruntime::types::RetrieveAndGenerateType::KnowledgeBase)
        .knowledge_base_configuration(kb_config)
        .build()?;

    let rag = agentruntime
        .retrieve_and_generate()
        .input(rag_input)
        .retrieve_and_generate_configuration(rag_config)
        .send()
        .await;

    match rag {
        Ok(output) => {
            let text = get_rag_output_text(output)?;
            println!("{}", text);
            Ok(())
        }
        Err(e) => Err(e
            .as_service_error()
            .map(BedrockConverseError::from)
            .unwrap_or_else(|| BedrockConverseError("Unknown service error".into()))),
    }?;

    println!();

    Ok(())
}
