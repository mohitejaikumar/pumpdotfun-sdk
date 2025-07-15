use std::sync::Arc;

use anchor_lang::pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

pub mod instructions;
pub use instructions::*;
pub mod constants;
pub mod errors;
use crate::errors::ErrorCode;
pub mod pda;
pub mod states;

pub const PUMP_DOT_FUN_DEVENT_PROGRAM_ID: Pubkey =
    pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

pub struct PumpDotFunSdk {
    pub rpc: Arc<RpcClient>,
}

impl PumpDotFunSdk {
    pub fn new(rpc: Arc<RpcClient>) -> Self {
        Self { rpc }
    }

    pub fn create(
        &self,
        accounts: instructions::create::CreateAccounts,
        args: instructions::create::CreateArgs,
    ) -> Instruction {
        instructions::create::create_ix(&PUMP_DOT_FUN_DEVENT_PROGRAM_ID, accounts, args)
    }

    pub fn buy(
        &self,
        accounts: instructions::buy::BuyAccounts,
        args: instructions::buy::Buy,
    ) -> Result<Vec<Instruction>, ErrorCode> {
        instructions::buy::buy_ix(&self.rpc, &PUMP_DOT_FUN_DEVENT_PROGRAM_ID, accounts, args)
    }

    pub fn sell(
        &self,
        accounts: instructions::sell::SellAccounts,
        args: instructions::sell::Sell,
    ) -> Result<Vec<Instruction>, ErrorCode> {
        instructions::sell::sell_ix(&self.rpc, &PUMP_DOT_FUN_DEVENT_PROGRAM_ID, accounts, args)
    }
}
