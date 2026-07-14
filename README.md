# Bitcoin RPC CLI

A command-line application written in Rust that communicates with a local Bitcoin Core node running on Regtest using the Bitcoin Core JSON-RPC API.

It provides a clean, modular interface for interacting with Bitcoin Core while demonstrating idiomatic Rust, strongly typed JSON-RPC responses, and reusable client abstractions.

---

## Features

- Connects to a local Bitcoin Core node running on Regtest
- Configuration via environment variables
- Strongly typed JSON-RPC response models
- Reusable JSON-RPC client abstraction
- Graceful error handling using `anyhow`
- Modular command structure
- Pretty terminal output

Supported commands:

- `blockchain-info`
- `wallet-info`
- `balance`
- `new-address`
- `rpc <method> [params...]`

---

## Project Structure

```
src/
├── main.rs
├── cli.rs
├── config.rs
├── error.rs
├── models.rs
├── rpc.rs
└── commands/
    ├── address.rs
    ├── blockchain.rs
    ├── rpc.rs
    └── wallet.rs
```

Each module has a single responsibility:

- **main.rs** – CLI entry point and command dispatch.
- **cli.rs** – Command-line interface definitions using Clap.
- **config.rs** – Loads RPC configuration from environment variables.
- **rpc.rs** – Reusable Bitcoin Core JSON-RPC client.
- **models.rs** – Strongly typed response models.
- **error.rs** – Bitcoin Core RPC error models.
- **commands/** – Individual command implementations.

---

# Architecture

```
Command Line
      │
      ▼
Command Handler
      │
      ▼
RpcClient
      │
      ▼
Bitcoin Core JSON-RPC
      │
      ▼
Bitcoin Core (Regtest)
```

---

## Requirements

- Rust
- Cargo
- Docker Desktop
- Polar
- Bitcoin Core

---

# Installation

Clone the repository.

```bash
git clone <repository-url>
cd bitcoin_rpc_cli
```

Install dependencies.

```bash
cargo build
```

---

# Setting up Polar

1. Install Polar from the official website:

   https://lightningpolar.com/

2. Create a new Bitcoin network.

3. Add a Bitcoin Core node.

4. Start the network.

5. Open the node settings and copy:

- RPC URL
- RPC Username
- RPC Password

6. Create a `.env` file in the project root.

```env
RPC_URL=http://127.0.0.1:18443
RPC_USER=bitcoin
RPC_PASSWORD=password
```

---

# Running the application

Blockchain information

```bash
cargo run -- blockchain-info
```

Wallet information

```bash
cargo run -- wallet-info
```

Wallet balance

```bash
cargo run -- balance
```

Generate a new address

```bash
cargo run -- new-address
```

Execute any Bitcoin Core RPC command

```bash
cargo run -- rpc getblockcount
```

Example with parameters

```bash
cargo run -- rpc getblockhash 1
```

---

# Example Output

### blockchain-info

```text
╭──────────────────────────────────────────────╮
│            Blockchain Information            │
╰──────────────────────────────────────────────╯
Chain:                    regtest
Blocks:                   1
Headers:                  1
Difficulty:               0.00000000046565423739069247
Verification Progress:    17.85%
```

Additional examples:

- [`blockchain-info.txt`](examples/blockchain-info.txt)
- [`wallet-info.txt`](examples/wallet-info.txt)
- [`balance.txt`](examples/balance.txt)
- [`new-address.txt`](examples/new-address.txt)
- [`rpc-getblockcount.txt`](examples/rpc-getblockcount.txt)
- [`rpc-invalid-method.txt`](examples/rpc-invalid-method.txt)


---

# Error Handling

The application gracefully handles:

- Missing environment variables
- Authentication failures
- Connection failures
- Invalid JSON-RPC methods
- Invalid RPC parameters
- JSON deserialization failures

Errors are propagated using `anyhow` with contextual error messages.

Example:

```
Error: RPC Error (-32601): Method not found
```

---

# Design Decisions

## Strongly Typed Responses

Where appropriate, Bitcoin Core JSON-RPC responses are deserialized into strongly typed Rust structs instead of relying on generic JSON values. This improves type safety and makes the application easier to maintain.

## Reusable RPC Client

All communication with Bitcoin Core is encapsulated within a reusable `RpcClient`, which is responsible for:

- Authentication
- Sending HTTP requests
- JSON-RPC serialization
- Response deserialization
- RPC error handling

This keeps command implementations focused solely on application logic.


---

# Future Improvements

Possible future enhancements include:

- Configuration file support
- Multiple wallet support
- Structured logging with `tracing`
- Additional Bitcoin Core RPC commands
- Unit and integration tests

---

# Built With

- Rust
- Tokio
- Clap
- Reqwest
- Serde
- Anyhow
- Dotenvy
- Bitcoin Core
- Polar