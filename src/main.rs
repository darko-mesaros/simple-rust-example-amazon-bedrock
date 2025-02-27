use aws_sdk_bedrockruntime::types::ContentBlock;
use aws_sdk_bedrockruntime::types::InferenceConfiguration;
use aws_sdk_bedrockruntime::types::Message;
use aws_sdk_bedrockruntime::types::ConversationRole;

use aws_sdk_bedrockruntime::types::SystemContentBlock;
// Some helper functions in lib.rs
use hello_bedrock::BedrockConverseStreamError;
use hello_bedrock::get_converse_output_text;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = hello_bedrock::configure_aws("us-east-1".into(),None).await;
    let bedrockruntime = aws_sdk_bedrockruntime::Client::new(&config);

    let system_message = SystemContentBlock::Text(String::from("You are a helpful assistant."));

    let inference_config = InferenceConfiguration::builder()
        .top_p(0.8)
        .temperature(0.5)
        .max_tokens(2048)
        .build();

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(String::from("How many stacked 5.25 floppy disks would it take to get to the moon?")))
        .build()?;

    let response = bedrockruntime.converse_stream()
        .messages(message)
        .model_id("us.anthropic.claude-3-7-sonnet-20250219-v1:0")
        .inference_config(inference_config)
        .system(system_message)
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
                let next = get_converse_output_text(text)?;
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
