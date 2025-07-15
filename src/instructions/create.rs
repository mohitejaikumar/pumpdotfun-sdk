use crate::{
    constants::{
        ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, MPL_TOKEN_METADATA, RENT_SYSVAR, SYSTEM_PROGRAM,
        TOKEN_PROGRAM,
    },
    pda::get_bonding_curve_pda,
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
    pub user: Pubkey,
}

pub fn create_ix(program_id: &Pubkey, accounts: CreateAccounts, args: CreateArgs) -> Instruction {
    let discriminator = [24, 30, 200, 40, 5, 28, 7, 119];
    let mut data = discriminator.to_vec();
    data.extend(args.try_to_vec().unwrap());

    let bonding_curve = get_bonding_curve_pda(&accounts.mint);
    let associated_bonding_curve = get_associated_bonding_curve(&accounts.mint);
    let metadata_pda = get_metadata_pda(&accounts.mint);
    let global_pda = get_global_pda(&accounts.mint);

    let accounts = vec![
        AccountMeta::new(accounts.mint, true),
        AccountMeta::new(bonding_curve, false),
        AccountMeta::new(associated_bonding_curve, true),
        AccountMeta::new(global_pda, false),
        AccountMeta::new_readonly(MPL_TOKEN_METADATA, false),
        AccountMeta::new(metadata_pda, true),
        AccountMeta::new(accounts.user, true),
        AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(RENT_SYSVAR, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMP_DOT_FUN_DEVENT_PROGRAM_ID, false),
    ];

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}
