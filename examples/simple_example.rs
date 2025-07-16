use pumpdotfun_sdk::{
    instructions::{
        buy::{Buy, BuyAccounts},
        create::{CreateAccounts, CreateArgs},
    },
    PumpDotFunSdk, Sell, SellAccounts,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use std::sync::Arc;

/// Simple example showing basic pump.fun SDK usage
///
/// Usage:
/// 1. Save your wallet as ~/.config/solana/devnet-wallet.json
/// 2. Make sure you have devnet SOL: solana airdrop 2 --url devnet
/// 3. Run: cargo run --bin simple_example
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Simple pump.fun SDK Example");

    // Connect to devnet
    let rpc_client = Arc::new(RpcClient::new("https://api.devnet.solana.com".to_string()));
    let sdk = PumpDotFunSdk::new(rpc_client.clone());

    // Load your wallet (adjust path as needed)
    let wallet_path = std::env::var("SOLANA_WALLET").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        format!("{}/.config/solana/devnet-wallet.json", home)
    });

    let user_keypair = match read_keypair_file(&wallet_path) {
        Ok(keypair) => keypair,
        Err(_) => {
            println!("❌ Could not load wallet from {}", wallet_path);
            println!(
                "💡 Generate one with: solana-keygen new --outfile {}",
                wallet_path
            );
            println!("💡 Fund it with: solana airdrop 2 --url devnet");
            return Ok(());
        }
    };

    println!("✅ Loaded wallet: {}", user_keypair.pubkey());

    // Check balance
    let balance = rpc_client.get_balance(&user_keypair.pubkey())?;
    println!(
        "💰 Balance: {:.4} SOL",
        balance as f64 / LAMPORTS_PER_SOL as f64
    );

    if balance < LAMPORTS_PER_SOL / 10 {
        println!("⚠️  Low balance! Run: solana airdrop 2 --url devnet");
        return Ok(());
    }

    // Example: Create a new token
    println!("\n📝 Creating a new token...");
    let mint_pubkey = create_simple_token(&sdk, &user_keypair)?;

    // Example: Buy some tokens
    println!("\n💰 Buying tokens...");
    buy_simple_tokens(&sdk, &user_keypair, &mint_pubkey)?;

    println!("\n🎉 Simple example completed! Check the transactions on Solana Explorer (devnet)");

    println!("\n💰 Selling tokens...");
    sell_simple_tokens(&sdk, &user_keypair, &mint_pubkey)?;

    Ok(())
}

fn create_simple_token(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let accounts = CreateAccounts {
        mint: mint_pubkey,
        user: user_keypair.pubkey(),
    };

    let args = CreateArgs {
        name: "Test Pump Token".to_string(),
        symbol: "TEST".to_string(),
        uri: "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png".to_string(),
        creator: user_keypair.pubkey(),
    };

    let instruction = sdk.create(accounts, args);
    let recent_blockhash = sdk.rpc.get_latest_blockhash()?;

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&user_keypair.pubkey()));
    transaction.sign(&[user_keypair, &mint_keypair], recent_blockhash);

    let signature = sdk.rpc.send_and_confirm_transaction(&transaction)?;
    println!(
        "✅ Token created! Mint: {} | TX: {}",
        mint_pubkey, signature
    );

    Ok(mint_pubkey)
}

fn buy_simple_tokens(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
    mint: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = BuyAccounts {
        mint: *mint,
        user: user_keypair.pubkey(),
    };

    let args = Buy {
        amount: 100_000_000,                   // 0.1 tokens (assuming 9 decimals)
        max_sol_cost: LAMPORTS_PER_SOL / 1000, // Max 0.001 SOL
        slippage: 10,                          // 10% slippage
    };

    let instructions = sdk
        .buy(accounts, args)
        .map_err(|e| format!("Buy error: {:?}", e))?;
    let recent_blockhash = sdk.rpc.get_latest_blockhash()?;

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&user_keypair.pubkey()));
    transaction.sign(&[user_keypair], recent_blockhash);

    let signature = sdk.rpc.send_and_confirm_transaction(&transaction)?;
    println!("✅ Tokens purchased! TX: {}", signature);

    Ok(())
}

fn sell_simple_tokens(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
    mint: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let accounts = SellAccounts {
        mint: *mint,
        user: user_keypair.pubkey(),
    };

    let args = Sell {
        amount: 50_000_000,                         // 0.05 tokens (assuming 9 decimals)
        min_sol_output: LAMPORTS_PER_SOL / 1000000, 
        slippage: 10,                               // 10% slippage
    };

    let instructions = sdk
        .sell(accounts, args)
        .map_err(|e| format!("Sell error: {:?}", e))?;
    let recent_blockhash = sdk.rpc.get_latest_blockhash()?;

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&user_keypair.pubkey()));
    transaction.sign(&[user_keypair], recent_blockhash);

    let signature = sdk.rpc.send_and_confirm_transaction(&transaction)?;
    println!("✅ Tokens sold! TX: {}", signature);

    Ok(())
}
