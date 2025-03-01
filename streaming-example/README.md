# Hello Bedrock

This is a simple Rust application that demonstrates how to use Amazon Bedrock's streaming API to interact with large language models.

## Overview

This project showcases:
- How to set up and configure AWS credentials for Bedrock
- How to create a streaming conversation with a Bedrock model
- How to process and display the streamed responses

## Features

- Supports multiple Bedrock models (Claude, Llama, etc.)
- Streams responses in real-time
- Configurable AWS settings

## Usage

1. Ensure you have the necessary AWS credentials set up.
2. Choose your preferred model by uncommenting the appropriate `MODEL_ID` in `src/main.rs`.
3. Run the application:

   ```
   cargo run
   ```

4. Enter your prompt when asked.

## Requirements

- Rust 2024 edition
- AWS account with access to Amazon Bedrock

## Note

This is a sample project intended for demonstration purposes. It showcases basic integration with Amazon Bedrock and should not be used in production without further development and security considerations.
