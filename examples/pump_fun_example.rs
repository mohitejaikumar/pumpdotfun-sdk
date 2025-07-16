use std::sync::Arc;

use pumpdotfun_sdk::{
    instructions::{
        buy::{Buy, BuyAccounts},
        create::{CreateAccounts, CreateArgs},
        sell::{Sell, SellAccounts},
    },
    PumpDotFunSdk,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

/// Example program demonstrating pump.fun SDK usage on devnet
///
/// This program shows how to:
/// 1. Create a new token with bonding curve
/// 2. Buy tokens from the bonding curve
/// 3. Sell tokens back to the bonding curve
///
/// Before running, make sure you have:
/// - A devnet wallet with SOL for transaction fees
/// - The devnet RPC endpoint configured
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Pump.fun SDK Example - Devnet Interaction");
    println!("===============================================");

    // Configuration for devnet
    let devnet_url = "https://api.devnet.solana.com";
    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        devnet_url.to_string(),
        CommitmentConfig::confirmed(),
    ));

    // Initialize the pump.fun SDK
    let sdk = PumpDotFunSdk::new(rpc_client.clone());
    println!("✅ Connected to Solana devnet");

    // Load your wallet keypair (replace with your actual keypair)
    // For this example, we'll generate a new keypair
    let user_keypair = Keypair::new();
    let user_pubkey = user_keypair.pubkey();

    println!("👤 User public key: {}", user_pubkey);

    // Check user's SOL balance
    match rpc_client.get_balance(&user_pubkey) {
        Ok(balance) => {
            let sol_balance = balance as f64 / LAMPORTS_PER_SOL as f64;
            println!("💰 User SOL balance: {:.4} SOL", sol_balance);

            if balance < LAMPORTS_PER_SOL / 10 {
                // Less than 0.1 SOL
                println!("⚠️  Warning: Low SOL balance. You may need to airdrop SOL for transaction fees.");
                println!("   Run: solana airdrop 2 {} --url devnet", user_pubkey);
                return Ok(());
            }
        }
        Err(e) => {
            println!("❌ Failed to get balance: {}", e);
            return Err(e.into());
        }
    }

    // Example 1: Create a new token
    println!("\n📝 Example 1: Creating a new token");
    println!("==================================");

    let token_example = create_token_example(&sdk, &user_keypair)?;
    let mint_pubkey = token_example.mint;

    // Example 2: Buy tokens
    println!("\n💰 Example 2: Buying tokens");
    println!("===========================");

    buy_tokens_example(&sdk, &user_keypair, &mint_pubkey)?;

    // Example 3: Sell tokens
    println!("\n💸 Example 3: Selling tokens");
    println!("============================");

    sell_tokens_example(&sdk, &user_keypair, &mint_pubkey)?;

    println!("\n🎉 All examples completed successfully!");
    Ok(())
}

/// Example of creating a new token with pump.fun
struct TokenCreationResult {
    mint: Pubkey,
    signature: Signature,
}

fn create_token_example(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
) -> Result<TokenCreationResult, Box<dyn std::error::Error>> {
    // Generate a new mint keypair
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    println!("🪙 Creating token with mint: {}", mint_pubkey);

    // Set up accounts for token creation
    let create_accounts = CreateAccounts {
        mint: mint_pubkey,
        user: user_keypair.pubkey(),
    };

    // Set up token metadata
    let create_args = CreateArgs {
        name: "My Pump Token".to_string(),
        symbol: "PUMP".to_string(),
        uri: "https://raw.githubusercontent.com/example/metadata.json".to_string(), // Replace with actual metadata URI
        creator: user_keypair.pubkey(),
    };

    // Create the instruction
    let create_instruction = sdk.create(create_accounts, create_args);

    // Build and send transaction
    let recent_blockhash = sdk.rpc.get_latest_blockhash()?;
    let mut transaction =
        Transaction::new_with_payer(&[create_instruction], Some(&user_keypair.pubkey()));
    transaction.sign(&[user_keypair, &mint_keypair], recent_blockhash);

    // Send transaction
    match sdk.rpc.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Token created successfully!");
            println!("   Mint: {}", mint_pubkey);
            println!("   Transaction: {}", signature);

            Ok(TokenCreationResult {
                mint: mint_pubkey,
                signature,
            })
        }
        Err(e) => {
            println!("❌ Failed to create token: {}", e);
            Err(e.into())
        }
    }
}

/// Example of buying tokens from a bonding curve
fn buy_tokens_example(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
    mint: &Pubkey,
) -> Result<Signature, Box<dyn std::error::Error>> {
    println!("🛒 Buying tokens from mint: {}", mint);

    // Set up accounts for buying
    let buy_accounts = BuyAccounts {
        mint: *mint,
        user: user_keypair.pubkey(),
    };

    // Set up buy parameters
    let buy_args = Buy {
        amount: 1_000_000_000, // Amount of tokens to buy (adjust based on token decimals)
        max_sol_cost: LAMPORTS_PER_SOL / 100, // Maximum 0.01 SOL to spend
        slippage: 500,         // 5% slippage tolerance (in basis points)
    };

    // Create buy instructions
    match sdk.buy(buy_accounts, buy_args) {
        Ok(instructions) => {
            println!(
                "📦 Created {} instruction(s) for buy transaction",
                instructions.len()
            );

            // Build and send transaction
            let recent_blockhash = sdk.rpc.get_latest_blockhash()?;
            let mut transaction =
                Transaction::new_with_payer(&instructions, Some(&user_keypair.pubkey()));
            transaction.sign(&[user_keypair], recent_blockhash);

            match sdk.rpc.send_and_confirm_transaction(&transaction) {
                Ok(signature) => {
                    println!("✅ Tokens purchased successfully!");
                    println!("   Transaction: {}", signature);
                    Ok(signature)
                }
                Err(e) => {
                    println!("❌ Failed to buy tokens: {}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create buy instructions: {:?}", e);
            Err(format!("Buy instruction error: {:?}", e).into())
        }
    }
}

/// Example of selling tokens to a bonding curve
fn sell_tokens_example(
    sdk: &PumpDotFunSdk,
    user_keypair: &Keypair,
    mint: &Pubkey,
) -> Result<Signature, Box<dyn std::error::Error>> {
    println!("💸 Selling tokens for mint: {}", mint);

    // Set up accounts for selling
    let sell_accounts = SellAccounts {
        mint: *mint,
        user: user_keypair.pubkey(),
    };

    // Set up sell parameters
    let sell_args = Sell {
        amount: 500_000_000, // Amount of tokens to sell (half of what we bought)
        min_sol_output: LAMPORTS_PER_SOL / 1000, // Minimum 0.001 SOL expected
        slippage: 500,       // 5% slippage tolerance (in basis points)
    };

    // Create sell instructions
    match sdk.sell(sell_accounts, sell_args) {
        Ok(instructions) => {
            println!(
                "📦 Created {} instruction(s) for sell transaction",
                instructions.len()
            );

            // Build and send transaction
            let recent_blockhash = sdk.rpc.get_latest_blockhash()?;
            let mut transaction =
                Transaction::new_with_payer(&instructions, Some(&user_keypair.pubkey()));
            transaction.sign(&[user_keypair], recent_blockhash);

            match sdk.rpc.send_and_confirm_transaction(&transaction) {
                Ok(signature) => {
                    println!("✅ Tokens sold successfully!");
                    println!("   Transaction: {}", signature);
                    Ok(signature)
                }
                Err(e) => {
                    println!("❌ Failed to sell tokens: {}", e);
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create sell instructions: {:?}", e);
            Err(format!("Sell instruction error: {:?}", e).into())
        }
    }
}

/// Utility function to load keypair from file
/// Usage: load_keypair_from_file("/path/to/your/wallet.json")
#[allow(dead_code)]
fn load_keypair_from_file(filepath: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    let keypair_data = std::fs::read_to_string(filepath)?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_data)?;
    Ok(Keypair::from_bytes(&keypair_bytes)?)
}

/// Utility function to get token balance
#[allow(dead_code)]
fn get_token_balance(
    rpc_client: &RpcClient,
    user_pubkey: &Pubkey,
    mint: &Pubkey,
) -> Result<u64, Box<dyn std::error::Error>> {
    use spl_associated_token_account::get_associated_token_address;

    let ata = get_associated_token_address(user_pubkey, mint);

    match rpc_client.get_token_account_balance(&ata) {
        Ok(balance) => Ok(balance.amount.parse()?),
        Err(_) => Ok(0), // Account doesn't exist, balance is 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = Keypair::new();
        assert!(!keypair.pubkey().to_string().is_empty());
    }

    #[test]
    fn test_sdk_initialization() {
        let rpc_client = Arc::new(RpcClient::new("https://api.devnet.solana.com".to_string()));
        let sdk = PumpDotFunSdk::new(rpc_client);
        // SDK should be created successfully
        assert!(!sdk.rpc.url().is_empty());
    }
}
