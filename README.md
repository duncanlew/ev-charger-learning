# Welcome to your CDK TypeScript project

This is a blank project for CDK development with TypeScript.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   type-check the project
* `npm run watch`   watch for changes and type-check
* `npm run test`    perform the jest unit tests
* `npx cdk deploy`  deploy this stack to your default AWS account/region
* `npx cdk diff`    compare deployed stack with current state
* `npx cdk synth`   emits the synthesized CloudFormation template


## Run the Rust CLI

From the repository root, run:

```bash
cargo run --manifest-path lambda/availability/Cargo.toml --bin cli
```

Alternatively, change to the Rust package directory first:

```bash
cd lambda/availability
cargo run --bin cli
```

## Todo for next time
2026-08-26: add the availability response in cli
2026-08-30: Use serde_json::to_string_pretty, and write some example tests
