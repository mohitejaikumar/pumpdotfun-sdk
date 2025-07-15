use solana_client::rpc_client::RpcClient;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

pub mod instructions;
pub use instructions::*;
pub mod constants;
pub mod pda;

pub const PUMP_DOT_FUN_DEVENT_PROGRAM_ID: Pubkey =
    Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

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
}
