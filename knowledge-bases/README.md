# Hello Bedrock

This is a simple Rust project that demonstrates interaction with Amazon Bedrock and its knowledge base feature.

## Features

- Connects to AWS Bedrock service
- Uses a specified AI model for text generation
- Interacts with a knowledge base to retrieve and generate information
- Supports various AWS credential providers

## Prerequisites

- Rust programming environment
- AWS account with access to Bedrock
- Proper AWS credentials configured

## Setup

1. Clone the repository
2. Create a `.env` file in the project root and add your `KB_ID`:
   ```
   KB_ID=your_knowledge_base_id_here
   ```
3. Run `cargo build` to compile the project

## Usage

Run the project with:

```
cargo run
```

When prompted, enter your question or prompt. The application will use the specified AI model and knowledge base to generate a response.

## Configuration

- The AI model can be changed by uncommenting the desired `MODEL_ID` in `src/main.rs`
- AWS region and profile can be configured in the code or through environment variables

## Note

This is a sample project intended for demonstration purposes. It showcases basic integration with Amazon Bedrock and should not be used in production without further development and security considerations.
