# Getting Started with pump.fun SDK

Welcome! This guide will get you up and running with the pump.fun SDK in just a few minutes.

## 🚀 Quick Start (5 minutes)

### Step 1: Clone and Setup
```bash
# Clone the repository
git clone <your-repo-url>
cd pumpdotfun-sdk

# Run the automatic setup script
./scripts/setup.sh
```

The setup script will:
- Install Solana CLI (if needed)
- Create a devnet wallet
- Fund it with 2 SOL for testing
- Configure everything for you

### Step 2: Run Your First Example
```bash
# Build the project
cargo build

# Run the simple example
cargo run --bin simple_example
```

🎉 **That's it!** You just created a token and bought some on pump.fun devnet!

## 📝 What Just Happened?

The simple example:
1. ✅ Connected to Solana devnet
2. 🪙 Created a new token called "Test Pump Token" (TEST)
3. 💰 Bought 0.1 tokens using your devnet SOL
4. 📋 Displayed the transaction signatures

You can verify the transactions on [Solana Explorer](https://explorer.solana.com/?cluster=devnet) by pasting the transaction IDs.

## 🔗 View Your Transactions

After running the example, you'll see output like:
```
✅ Token created! Mint: AbC1234... | TX: def5678...
✅ Tokens purchased! TX: ghi9012...
```

Copy those transaction IDs and view them on [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet).

## 🛠️ What's Next?

### Option 1: Run the Full Example
```bash
cargo run --bin pump_fun_example
```
This example shows token creation, buying, AND selling.

### Option 2: Build Your Own Program
Check out the code structure:
- `src/lib.rs` - Main SDK
- `examples/simple_example.rs` - Basic usage
- `examples/pump_fun_example.rs` - Advanced usage

### Option 3: Integrate into Your Project
Add to your `Cargo.toml`:
```toml
[dependencies]
pumpdotfun-sdk = { path = "path/to/pumpdotfun-sdk" }
solana-client = "2.3.4"
solana-sdk = "2.3.1"
```

## 🧰 Core Operations

### Create a Token
```rust
let sdk = PumpDotFunSdk::new(rpc_client);
let instruction = sdk.create(accounts, args);
```

### Buy Tokens
```rust
let instructions = sdk.buy(accounts, buy_args)?;
```

### Sell Tokens
```rust
let instructions = sdk.sell(accounts, sell_args)?;
```

## 🔍 Understanding the Output

When you run the examples, you'll see:
- **Wallet address**: Your devnet wallet public key
- **Balance**: Your SOL balance (should be ~2 SOL after setup)
- **Mint address**: The new token's unique identifier
- **Transaction IDs**: Blockchain transaction signatures

## ⚠️ Important Notes

- **This is DEVNET**: Use fake SOL, no real money involved
- **Slippage**: We use 10% slippage for safety (price can vary ±10%)
- **Gas fees**: Each transaction costs a tiny amount of SOL (~0.00025 SOL)
- **Token decimals**: Tokens typically use 9 decimal places

## 🆘 Troubleshooting

### "Could not load wallet"
```bash
# The setup script should create this, but if needed:
solana-keygen new --outfile ~/.config/solana/devnet-wallet.json
```

### "Low balance"
```bash
# Get more devnet SOL:
solana airdrop 2 --url devnet
```

### "RPC errors"
- Wait a moment and try again (devnet can be slow)
- Check your internet connection

### Build errors
```bash
# Update Rust:
rustup update

# Clean and rebuild:
cargo clean && cargo build
```

## 📚 Learn More

- Read the full [README.md](README.md) for detailed documentation
- Explore the [examples/](examples/) directory for more code samples
- Check the [src/](src/) directory to understand the SDK internals

## 🤝 Need Help?

- Create an issue in this repository
- Check the [Solana documentation](https://docs.solana.com/)
- Visit the [Anchor documentation](https://book.anchor-lang.com/)

---

Ready to build something awesome with pump.fun? 🚀 