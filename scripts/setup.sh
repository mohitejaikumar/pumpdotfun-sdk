#!/bin/bash

# Setup script for pump.fun SDK
echo "🚀 Setting up pump.fun SDK for devnet..."

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Installing..."
    sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
    echo "✅ Solana CLI installed"
    echo "⚠️  Please restart your terminal or run: source ~/.bashrc"
    exit 1
fi

# Set Solana to devnet
echo "🔧 Configuring Solana CLI for devnet..."
solana config set --url devnet

# Create wallet directory if it doesn't exist
mkdir -p ~/.config/solana

# Check if wallet exists, create if not
WALLET_PATH="$HOME/.config/solana/devnet-wallet.json"
if [ ! -f "$WALLET_PATH" ]; then
    echo "🔑 Creating new devnet wallet..."
    solana-keygen new --outfile "$WALLET_PATH" --no-bip39-passphrase
else
    echo "✅ Found existing wallet at $WALLET_PATH"
fi

# Show wallet address
WALLET_ADDRESS=$(solana-keygen pubkey "$WALLET_PATH")
echo "👤 Wallet address: $WALLET_ADDRESS"

# Check balance
BALANCE=$(solana balance --keypair "$WALLET_PATH" 2>/dev/null || echo "0 SOL")
echo "💰 Current balance: $BALANCE"

# Airdrop SOL if balance is low
if [[ "$BALANCE" == "0 SOL" ]] || [[ "$BALANCE" =~ ^0\.0 ]]; then
    echo "💸 Requesting airdrop (2 SOL)..."
    solana airdrop 2 --keypair "$WALLET_PATH"
    echo "✅ Airdrop complete"
    
    # Check new balance
    NEW_BALANCE=$(solana balance --keypair "$WALLET_PATH")
    echo "💰 New balance: $NEW_BALANCE"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Build the project: cargo build"
echo "2. Run simple example: cargo run --bin simple_example"
echo "3. Run full example: cargo run --bin pump_fun_example"
echo ""
echo "Your wallet: $WALLET_ADDRESS"
echo "Wallet file: $WALLET_PATH"
echo ""
echo "📚 Read the README.md for more information" 