# Pump.fun SDK for Rust

A Rust SDK for interacting with pump.fun on Solana. This SDK provides easy-to-use functions for creating tokens, buying from bonding curves, and selling to bonding curves.

## Features

- 🪙 **Create new tokens** with metadata and bonding curves
- 💰 **Buy tokens** from bonding curves with slippage protection
- 💸 **Sell tokens** to bonding curves with slippage protection
- 🔧 **Built-in utilities** for account management and PDA derivation
- ✅ **Type-safe** Rust implementation using Anchor framework

## Program Information

- **Program ID**: `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- **Network**: Solana Mainnet
- **Framework**: Anchor

## Installation

Add this SDK to your `Cargo.toml`:

```toml
[dependencies]
pumpdotfun-sdk = { path = "." }
solana-client = "2.3.4"
solana-sdk = "2.3.1"
```

## Quick Start

### 1. Initialize the SDK

```rust
use std::sync::Arc;
use pumpdotfun_sdk::PumpDotFunSdk;
use solana_client::rpc_client::RpcClient;

let rpc_client = Arc::new(RpcClient::new("https://api.devnet.solana.com".to_string()));
let sdk = PumpDotFunSdk::new(rpc_client);
```

### 2. Create a new token

```rust
use pumpdotfun_sdk::instructions::create::{CreateAccounts, CreateArgs};
use solana_sdk::{signature::Keypair, signer::Signer};

let mint_keypair = Keypair::new();
let user_keypair = Keypair::new(); // Your wallet keypair

let create_accounts = CreateAccounts {
    mint: mint_keypair.pubkey(),
    user: user_keypair.pubkey(),
};

let create_args = CreateArgs {
    name: "My Token".to_string(),
    symbol: "TOKEN".to_string(),
    uri: "https://example.com/metadata.json".to_string(),
    creator: user_keypair.pubkey(),
};

let instruction = sdk.create(create_accounts, create_args);
```

### 3. Buy tokens

```rust
use pumpdotfun_sdk::instructions::buy::{BuyAccounts, Buy};
use solana_sdk::native_token::LAMPORTS_PER_SOL;

let buy_accounts = BuyAccounts {
    mint: mint_pubkey,
    user: user_keypair.pubkey(),
};

let buy_args = Buy {
    amount: 100_000_000, // Amount of tokens to buy
    max_sol_cost: LAMPORTS_PER_SOL / 1000, // Maximum 0.001 SOL
    slippage: 10, // 10% slippage tolerance 
};

let instructions = sdk.buy(buy_accounts, buy_args)?;
```

### 4. Sell tokens

```rust
use pumpdotfun_sdk::instructions::sell::{SellAccounts, Sell};

let sell_accounts = SellAccounts {
    mint: mint_pubkey,
    user: user_keypair.pubkey(),
};

let sell_args = Sell {
    amount: 50_000_000,                         // 0.05 tokens (assuming 9 decimals)
    min_sol_output: LAMPORTS_PER_SOL / 1000000, 
    slippage: 10,  // 10% slippage
};

let instructions = sdk.sell(sell_accounts, sell_args)?;
```

## Running the Example

This repository includes a comprehensive example that demonstrates all SDK features.

### Quick Setup (Recommended)

Run the setup script to automatically configure everything:

```bash
# Make sure you have Rust installed first
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and run setup
git clone <your-repo>
cd pumpdotfun-sdk
./scripts/setup.sh
```

The setup script will:
- Install Solana CLI (if needed)
- Configure devnet
- Create a wallet (if needed) 
- Fund it with SOL
- Show you next steps

### Manual Prerequisites

If you prefer manual setup:

1. **Install Rust and Cargo**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Solana CLI**:
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

3. **Get devnet SOL**:
   ```bash
   # Generate a new keypair (or use existing one)
   solana-keygen new --outfile ~/.config/solana/devnet-wallet.json
   
   # Airdrop SOL to your wallet
   solana airdrop 2 --url devnet --keypair ~/.config/solana/devnet-wallet.json
   ```

### Running the Example

1. **Clone and build**:
   ```bash
   git clone <your-repo>
   cd pumpdotfun-sdk
   cargo build
   ```

2. **Run the examples**:

   **Simple Example** (recommended for beginners):
   ```bash
   cargo run --bin simple_example
   ```
   

The examples will:
1. Connect to Solana devnet
2. Create a new token with bonding curve
3. Buy tokens from the bonding curve
4. Sell some tokens back (full example only)
5. Display transaction signatures and results

> **Note**: The simple example uses your actual wallet from `~/.config/solana/devnet-wallet.json`, while the full example generates a new keypair each time (which won't have SOL).

## SDK Structure

```
src/
├── lib.rs              # Main SDK entry point
├── instructions/       # Instruction builders
│   ├── create.rs      # Token creation
│   ├── buy.rs         # Token purchasing
│   └── sell.rs        # Token selling
├── constants.rs        # Program constants
├── errors.rs          # Error definitions
├── pda.rs             # PDA derivation utilities
└── states/            # Account state definitions
    └── global.rs      # Global state structure
```


## Key Concepts

### Bonding Curves
pump.fun uses bonding curves to automatically provide liquidity for newly created tokens. As more tokens are bought, the price increases along the curve.

### Slippage Protection
Both buy and sell operations include slippage protection:
- **Buy**: Specify maximum SOL you're willing to spend
- **Sell**: Specify minimum SOL you're willing to receive
- **Slippage**: Tolerance (5 = 5%)

### Associated Token Accounts
The SDK automatically handles Associated Token Account (ATA) creation when needed for buy operations.

## Error Handling

The SDK includes comprehensive error handling:

```rust
use pumpdotfun_sdk::errors::ErrorCode;

match sdk.buy(accounts, args) {
    Ok(instructions) => {
        // Handle success
    }
    Err(ErrorCode::InvalidSlippage) => {
        println!("Slippage must be non-negative");
    }
    Err(ErrorCode::GlobalNotFound) => {
        println!("Global state not found - program may not be initialized");
    }
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request


## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This SDK is for educational and development purposes. Always test thoroughly on devnet before using on mainnet. The authors are not responsible for any financial losses.
