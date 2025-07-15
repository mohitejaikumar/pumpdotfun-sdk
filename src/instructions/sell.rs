use anchor_lang::prelude::*;
use core::result::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    constants::{ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, SYSTEM_PROGRAM, TOKEN_PROGRAM},
    errors::ErrorCode,
    pda::{get_associated_bonding_curve, get_bonding_curve_pda, get_global_pda},
    states::Global,
    PUMP_DOT_FUN_DEVENT_PROGRAM_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SellArgs {
    pub amount: u64,
    pub min_sol_output: u64,
}

pub struct Sell {
    pub amount: u64,
    pub min_sol_output: u64,
    pub slippage: i64,
}

pub struct SellAccounts {
    pub mint: Pubkey,
    pub user: Pubkey,
}

pub fn sell_ix(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    accounts: SellAccounts,
    args: Sell,
) -> Result<Vec<Instruction>, ErrorCode> {
    let discriminator = [51, 230, 133, 164, 1, 127, 131, 173];
    let mut data = discriminator.to_vec();

    if args.slippage < 0 {
        return Err(ErrorCode::InvalidSlippage);
    }

    let bonding_curve = get_bonding_curve_pda(&accounts.mint);
    let associated_bonding_curve = get_associated_bonding_curve(&accounts.mint);
    let global_pda = get_global_pda();
    let associated_user_token_account =
        get_associated_token_address(&accounts.user, &accounts.mint);

    let global_account_data = rpc_client
        .get_account_data(&global_pda)
        .map_err(|_| ErrorCode::GlobalNotFound)?;

    let global: Global = Global::try_from_slice(&global_account_data)
        .map_err(|_| ErrorCode::DeserializationError)?;

    let fee_recipient = global.fee_recipient;
    let mut instructions: Vec<Instruction> = vec![];

    let accounts_metas = vec![
        AccountMeta::new_readonly(global_pda, false),
        AccountMeta::new(fee_recipient, true),
        AccountMeta::new_readonly(accounts.mint, false),
        AccountMeta::new(bonding_curve, true),
        AccountMeta::new(associated_bonding_curve, true),
        AccountMeta::new(associated_user_token_account, true),
        AccountMeta::new(accounts.user, true),
        AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
        AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMP_DOT_FUN_DEVENT_PROGRAM_ID, false),
    ];

    let min_cost = args.min_sol_output as u128;
    let slippage = args.slippage as u128;

    let slippage_amount = min_cost
        .checked_mul(slippage)
        .and_then(|v| v.checked_mul(10)) // for basis points scaling
        .and_then(|v| v.checked_div(1000))
        .ok_or(ErrorCode::Overflow)?; // still u128

    let new_sol_amount: u64 = min_cost
        .checked_add(slippage_amount)
        .ok_or(ErrorCode::Overflow)?
        .try_into()
        .map_err(|_| ErrorCode::Overflow)?;

    let sell_args = SellArgs {
        amount: args.amount,
        min_sol_output: new_sol_amount,
    };

    data.extend(sell_args.try_to_vec().unwrap());

    let sell_instruction = Instruction {
        program_id: *program_id,
        accounts: accounts_metas,
        data,
    };

    instructions.push(sell_instruction);

    Ok(instructions)
}
