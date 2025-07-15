use anchor_lang::prelude::*;
use core::result::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use crate::{
    constants::{
        ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, RENT_SYSVAR, SYSTEM_PROGRAM, TOKEN_PROGRAM,
    },
    errors::ErrorCode,
    pda::{get_associated_bonding_curve, get_bonding_curve_pda, get_global_pda},
    states::Global,
    PUMP_DOT_FUN_DEVENT_PROGRAM_ID,
};

pub struct Buy {
    pub amount: u64,
    pub max_sol_cost: u64,
    pub slippage: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyArgs {
    pub amount: u64,
    pub max_sol_cost: u64,
}

pub struct BuyAccounts {
    pub mint: Pubkey,
    pub user: Pubkey,
}

pub fn buy_ix(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    accounts: BuyAccounts,
    args: Buy,
) -> Result<Vec<Instruction>, ErrorCode> {
    if args.slippage < 0 {
        return Err(ErrorCode::InvalidSlippage);
    }

    let discriminator = [102, 6, 61, 18, 1, 218, 235, 234];
    let mut data = discriminator.to_vec();

    let bonding_curve = get_bonding_curve_pda(&accounts.mint);
    let associated_bonding_curve = get_associated_bonding_curve(&accounts.mint);
    let global_pda = get_global_pda();
    let associated_user_token_account =
        get_associated_token_address(&accounts.user, &accounts.mint);

    // Do rpc call and getaccount info

    // fetch the globalpda data

    // deserialize and get the fee_recepient address

    let global_account_data = rpc_client
        .get_account_data(&global_pda)
        .map_err(|_| ErrorCode::GlobalNotFound)?;

    let global: Global = Global::try_from_slice(&global_account_data)
        .map_err(|_| ErrorCode::DeserializationError)?;

    let fee_recipient = global.fee_recipient;

    // CHECK IF ATA exists

    let mut instructions: Vec<Instruction> = vec![];
    if rpc_client
        .get_account(&associated_user_token_account)
        .is_err()
    {
        let create_ata_ix = create_associated_token_account_idempotent(
            &accounts.user,
            &accounts.user,
            &accounts.mint,
            &ASSOCIATED_TOKEN_PROGRAM,
        );
        instructions.push(create_ata_ix);
    }

    let accounts_metas = vec![
        AccountMeta::new_readonly(global_pda, false),
        AccountMeta::new(fee_recipient, true),
        AccountMeta::new_readonly(accounts.mint, false),
        AccountMeta::new(bonding_curve, true),
        AccountMeta::new(associated_bonding_curve, true),
        AccountMeta::new(associated_user_token_account, true),
        AccountMeta::new(accounts.user, true),
        AccountMeta::new_readonly(SYSTEM_PROGRAM, false),
        AccountMeta::new_readonly(TOKEN_PROGRAM, false),
        AccountMeta::new_readonly(RENT_SYSVAR, false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMP_DOT_FUN_DEVENT_PROGRAM_ID, false),
    ];

    // Calculate the sol_amount_to_pay based on slippage

    let max_cost = args.max_sol_cost as u128;
    let slippage = args.slippage as u128;

    let slippage_amount = max_cost
        .checked_mul(slippage)
        .and_then(|v| v.checked_mul(10)) // for basis points scaling
        .and_then(|v| v.checked_div(1000))
        .ok_or(ErrorCode::Overflow)?; // still u128

    let new_sol_amount: u64 = max_cost
        .checked_add(slippage_amount)
        .ok_or(ErrorCode::Overflow)?
        .try_into()
        .map_err(|_| ErrorCode::Overflow)?;

    let buy_args = BuyArgs {
        amount: args.amount,
        max_sol_cost: new_sol_amount,
    };

    data.extend(buy_args.try_to_vec().unwrap());

    let buy_instruction = Instruction {
        program_id: *program_id,
        accounts: accounts_metas,
        data,
    };

    instructions.push(buy_instruction);

    Ok(instructions)
}
