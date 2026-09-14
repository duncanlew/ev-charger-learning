# Temporary. should be removed before merging
# Availability checker as a CLI-callable Rust function

## Context

The repo has a cargo-lambda scaffolded crate at `lambda/availability` (still "hello world") wired into a CDK stack via `RustFunction`. Duncan wants the availability-checking logic to be a plain Rust function runnable standalone from the CLI, printing a JSON response — with the lambda calling that same function later. First iteration is deliberately minimal: **no arguments at all**; the function returns a hardcoded placeholder response. Arguments come in a later iteration.

The clean Rust structure for "CLI now, lambda later, same core function" is one crate with a **library (the core function) + two binaries**.

## Approach

Restructure `lambda/availability` into a lib + bins crate. The lambda binary stays untouched for now; a new CLI binary calls the shared library function and prints its result as JSON.

```text
lambda/availability/
├── Cargo.toml            # add serde, serde_json
└── src/
    ├── lib.rs            # NEW: core check_availability() + response struct
    ├── main.rs           # existing lambda binary (unchanged for now)
    ├── http_handler.rs   # unchanged for now
    └── bin/
        └── cli.rs        # NEW: CLI entry point (no args)
```

Cargo conventions at work: `src/main.rs` is the default binary (`availability`, the lambda), `src/bin/cli.rs` auto-becomes a second binary named `cli`, and `src/lib.rs` is the shared library both link against. No `[[bin]]` config needed.

## Changes

### 1. `lambda/availability/Cargo.toml` — add serde deps

```toml
[package]
name = "availability"
version = "0.1.0"
edition = "2021"

[dependencies]
lambda_http = "1.0.0"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros"] }
```

Note: with two binaries and no `default-run` key, plain `cargo run` errors ("could not determine which binary to run") — always pass `--bin`: `cargo run --bin cli` for the CLI, `cargo run --bin availability` for the lambda binary. The `src/main.rs` (lambda) binary keeps the package name `availability`, i.e. it stays the primary/default binary of the crate.

### 2. `lambda/availability/src/lib.rs` — the core function (NEW)

```rust
use serde::Serialize;

/// Placeholder result of an availability check.
#[derive(Debug, Serialize)]
pub struct AvailabilityResponse {
    pub available: bool,
    pub message: String,
}

/// Core availability check. Currently a placeholder returning a fixed
/// response; later this will accept parameters and query real charger status.
pub fn check_availability() -> AvailabilityResponse {
    AvailabilityResponse {
        available: true,
        message: "Availability check placeholder".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_available_placeholder() {
        let response = check_availability();
        assert!(response.available);
    }

    #[test]
    fn serializes_to_json() {
        let json = serde_json::to_string(&check_availability()).unwrap();
        assert!(json.contains("\"available\":true"));
    }
}
```

### 3. `lambda/availability/src/bin/cli.rs` — CLI entry point (NEW)

```rust
use availability::check_availability;

fn main() {
    let response = check_availability();
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
}
```

### 4. `lib/ev-charger-learning-stack.ts` — pin the lambda binary (1 line)

The crate now has two binaries, so tell cargo-lambda-cdk which one is the lambda:

```ts
const availabilityFunction = new RustFunction(this, 'availabilityFunction', {
  manifestPath: './lambda/availability',
  binaryName: 'availability',   // <-- add
  runtime: 'provided.al2023',
  timeout: cdk.Duration.seconds(30),
});
```

No deploy needed now — this just keeps `cdk deploy` unambiguous for later.

### Later (out of scope now)

- Add parameters to `check_availability()` (e.g. a charger id), thread them through CLI args (clap or `std::env::args`).
- Make it `async fn` once it does real I/O.
- Wire the lambda: `http_handler.rs` calls `availability::check_availability()`, serializes with `serde_json`, returns as `application/json`.

## Background reading & resources

**Official docs on the `src/bin/` directory and crate layout:**

- [Package Layout — The Cargo Book](https://doc.rust-lang.org/cargo/guide/project-layout.html) — the canonical page: `src/main.rs` is the default binary, `src/bin/*.rs` are additional binaries, `src/lib.rs` is the library.
- [Cargo Targets — The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) — deeper reference on binary/library targets, auto-discovery, `default-run`, and explicit `[[bin]]` config.
- [How to Manage Multiple Binaries in One Cargo Project](https://www.rustfaq.org/en/how-to-manage-multiple-binaries-in-one-cargo-project/) — short practical FAQ on exactly our setup.

**Building CLI apps in Rust:**

- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html) — the official rust-cli book; its ["A command line app in 15 minutes"](https://rust-cli.github.io/book/tutorial/index.html) tutorial is a great next step (adds clap arg parsing, which is where this project goes in the next iteration).
- [The Rust Book, ch. 12: An I/O Project — Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) — **the closest match to this plan's structure**: it builds a CLI and then extracts the logic into `src/lib.rs` ("Separation of Concerns for Binary Projects"), for exactly the reason we're doing it — the core function becomes testable and reusable independent of the binary.
- *Command-Line Rust* by Ken Youens-Clark (O'Reilly) — a paid book, but well regarded if you want a project-based deep dive.

**Rust on AWS Lambda (the "later" part):**

- [Cargo Lambda docs](https://www.cargo-lambda.info/) — the tool cargo-lambda-cdk drives under the hood; explains `cargo lambda build`, `cargo lambda watch` (run the lambda locally!), and multi-binary packages.
- [aws-lambda-rust-runtime README](https://github.com/aws/aws-lambda-rust-runtime/blob/main/README.md) — the `lambda_http` crate your handler already uses, with examples.
- [Effortless Guide to Setting Up AWS Lambda with Rust](https://medium.com/@jed.lechner/effortless-guide-to-setting-up-aws-lambda-with-rust-b2630eeaa0f0) — walkthrough of the cargo-lambda flow end to end.

**Serde / JSON serialization:**

- [serde_json docs](https://docs.rs/serde_json) — `to_string_pretty`, `json!` macro, and derive examples right on the front page.
- [serde-rs/json on GitHub](https://github.com/serde-rs/json) — README shows the `#[derive(Serialize)]` struct → JSON string pattern this plan uses.

## Verification

From `lambda/availability/`:

1. `cargo test` — new lib tests + existing handler tests pass.
2. `cargo run --bin cli` — prints:

   ```json
   {
     "available": true,
     "message": "Availability check placeholder"
   }
   ```

3. From repo root: `npx cdk synth` still succeeds (confirms the `binaryName` change is valid).
