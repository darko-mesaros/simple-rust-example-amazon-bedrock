# Hello Bedrock

A simple Rust project demonstrating the use of Amazon Bedrock for AI model interactions with guardrails.

## Overview

This project showcases how to:
- Configure and use AWS SDK for Rust
- Interact with Amazon Bedrock Runtime
- Implement guardrails for AI model responses
- Stream responses from large language models

## Features

- Supports multiple AI models (Claude, Llama, etc.)
- Implements AWS credential management
- Uses environment variables for configuration
- Provides a simple command-line interface for user prompts

## Prerequisites

- Rust programming environment
- AWS account with access to Amazon Bedrock
- Proper AWS credentials configured

## Setup

1. Clone the repository
2. Create a `.env` file in the project root and add your `GUARDRAIL_ID`:
   ```
   GUARDRAIL_ID=your_guardrail_id_here
   ```
3. Run `cargo build` to compile the project

## Running the Project

```bash
cargo run
```

Follow the prompts to interact with the AI model.

## Note

This is a sample project intended for demonstration purposes. It showcases basic integration with Amazon Bedrock and should not be used in production without further development and security considerations.
