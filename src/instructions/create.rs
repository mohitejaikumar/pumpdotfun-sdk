use crate::constants::{
    ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, MPL_TOKEN_METADATA, RENT_SYSVAR, SYSTEM_PROGRAM,
    TOKEN_PROGRAM,
};
use anchor_lang::prelude::*;
use solana_sdk::instruction::{AccountMeta, Instruction};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Pubkey,
}

pub struct CreateAccounts {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub global: Pubkey,
    pub metadata: Pubkey,
    pub user: Pubkey,
    pub program: Pubkey,
}

pub fn create_ix(program_id: &Pubkey, accounts: CreateAccounts, args: CreateArgs) -> Instruction {
    let discriminator = [24, 30, 200, 40, 5, 28, 7, 119];
    let mut data = discriminator.to_vec();
    data.extend(args.try_to_vec().unwrap());

    let accounts = vec![
        AccountMeta::new(accounts.mint, true),
        AccountMeta::new(accounts.bonding_curve, false),
        AccountMeta::new(accounts.associated_bonding_curve, true),
        AccountMeta::new_readonly(accounts.global, false),
        AccountMeta::new_readonly(MPL_TOKEN_METADATA, false),
        AccountMeta::new(accounts.metadata, true),
        AccountMeta::new(accounts.user, true),
        AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(RENT_SYSVAR, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(accounts.program, false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
