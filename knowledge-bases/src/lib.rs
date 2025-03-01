use aws_config::environment::credentials::EnvironmentVariableCredentialsProvider;
use aws_config::imds::credentials::ImdsCredentialsProvider;
use aws_config::meta::credentials::CredentialsProviderChain;
use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::profile::ProfileFileRegionProvider;
use aws_sdk_bedrockagentruntime::operation::retrieve_and_generate::RetrieveAndGenerateError;
use aws_sdk_bedrockagentruntime::operation::retrieve_and_generate::RetrieveAndGenerateOutput;
use aws_sdk_bedrockruntime::operation::converse::ConverseError;
use aws_sdk_sts::Client;
use aws_types::region::Region;

use aws_config::BehaviorVersion;

use aws_types::SdkConfig;
// Make it pretty
use figlet_rs::FIGfont;

pub fn print_header(s: &str) {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(s);
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}

#[derive(Debug)]
pub struct BedrockConverseError(pub String);
impl std::fmt::Display for BedrockConverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke. Reason: {}", self.0)
    }
}
impl std::error::Error for BedrockConverseError {}
impl From<&str> for BedrockConverseError {
    fn from(value: &str) -> Self {
        BedrockConverseError(value.to_string())
    }
}
impl From<&ConverseError> for BedrockConverseError {
    fn from(value: &ConverseError) -> Self {
        BedrockConverseError::from(match value {
            ConverseError::ModelTimeoutException(_) => "Model took too long",
            ConverseError::ModelNotReadyException(_) => "Model is not ready",
            _ => "Unknown",
        })
    }
}
impl From<&RetrieveAndGenerateError> for BedrockConverseError {
    fn from(value: &RetrieveAndGenerateError) -> Self {
        BedrockConverseError::from(match value {
            RetrieveAndGenerateError::AccessDeniedException(_) => "Access Denied",
            RetrieveAndGenerateError::ThrottlingException(_) => "You are getting Throttled",
            _ => "Unknown",
        })
    }
}

pub fn get_rag_output_text(
    output: RetrieveAndGenerateOutput,
) -> Result<String, BedrockConverseError> {
    let text = output.output().ok_or("no output")?.text().to_string();
    Ok(text)
}

pub fn generate_arn(r: &str, a: &str, model: &str) -> Result<String, anyhow::Error> {
    //  if model starts with us. / eu. - use inference-profile
    //  else foundation-model
    let arn = if model.starts_with("us.") || model.starts_with("eu.") {
        format!("arn:aws:bedrock:{}:{}:inference-profile/{}", r, a, model)
    } else {
        // NOTE: This seems to be broken behavior, as when using foundation model ARNs we cannot
        // include the Account ID in the ARN.
        // format!("arn:aws:bedrock:{}:{}:foundation-model/{}",r, a, model)
        format!("arn:aws:bedrock:{}::foundation-model/{}", r, model)
    };

    Ok(arn)
}

pub async fn get_account_id(c: &SdkConfig) -> Result<String, anyhow::Error> {
    // Create an STS client
    let sts_client = Client::new(c);

    // Call GetCallerIdentity
    let identity = sts_client.get_caller_identity().send().await?;

    // Extract the account ID
    let account_id = identity
        .account()
        .ok_or(anyhow::anyhow!("Failed to extract Error"))?;

    Ok(account_id.to_string())
}

//======================================== AWS
pub async fn configure_aws(
    fallback_region: String,
    profile_name: Option<&String>,
) -> aws_config::SdkConfig {
    let profile = profile_name.map(|s| s.as_str()).unwrap_or("default");
    let region_provider = RegionProviderChain::first_try(
        ProfileFileRegionProvider::builder()
            .profile_name(profile)
            .build(),
    )
    .or_else(aws_config::environment::EnvironmentVariableRegionProvider::new())
    .or_else(aws_config::imds::region::ImdsRegionProvider::builder().build())
    .or_else(Region::new(fallback_region));

    let credentials_provider = CredentialsProviderChain::first_try(
        "Environment",
        EnvironmentVariableCredentialsProvider::new(),
    )
    .or_else(
        "Profile",
        ProfileFileCredentialsProvider::builder()
            .profile_name(profile)
            .build(),
    )
    .or_else("IMDS", ImdsCredentialsProvider::builder().build());

    aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(credentials_provider)
        .region(region_provider)
        .load()
        .await
}
//======================================== END AWS
