# Hello Bedrock 🦀

A simple Rust application demonstrating how to use the AWS SDK to interact with Amazon Bedrock.

## Overview

This project showcases how to:
- Configure AWS credentials and region settings
- Connect to Amazon Bedrock
- Stream responses from models using the Converse API

## Prerequisites

- Rust (latest stable version)
- AWS Account with access to Amazon Bedrock
- AWS credentials configured (either through environment variables, AWS profile, or IMDS)
- Access granted to a given model in your AWS account

## Configuration

The application supports multiple ways to configure AWS credentials:
1. Environment variables (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`)
2. AWS profile (default or specified)
3. IMDS (when running on AWS infrastructure)

## Installation

```bash
git clone [your-repo-url]
cd simple-rust-example-amazon-bedrock
cargo build
```

## Usage

Run the application with:

```bash
cargo run
```

The example code will:
1. Connect to Amazon Bedrock
2. Send a sample question to a model
3. Stream the response back to the console

## License

MIT License

## Security

This is a demonstration project. In a production environment, ensure you:
- Follow AWS security best practices
- Properly manage your AWS credentials
- Review and adjust the model parameters as needed
- Implement appropriate error handling and retry mechanisms

## Disclaimer

This is a demonstration project and may need additional error handling and security measures for production use.
