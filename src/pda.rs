use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

use crate::{constants::MPL_TOKEN_METADATA, PUMP_DOT_FUN_PROGRAM_ID};

const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";
const METADATA_SEED: &[u8] = b"metadata";

const GLOBAL_SEED: &[u8] = b"global";

const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

const CREATOR_VAULT_SEED: &[u8] = b"creator-vault";

pub fn get_bonding_curve_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[BONDING_CURVE_SEED, mint.to_bytes().as_slice()],
        &PUMP_DOT_FUN_PROGRAM_ID,
    )
    .0
}

pub fn get_mint_authority_pda() -> Pubkey {
    Pubkey::find_program_address(&[MINT_AUTHORITY_SEED], &PUMP_DOT_FUN_PROGRAM_ID).0
}

pub fn get_associated_bonding_curve(mint: &Pubkey) -> Pubkey {
    let bonding_curve = get_bonding_curve_pda(mint);
    get_associated_token_address(&bonding_curve, mint)
}

pub fn get_metadata_pda(mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            METADATA_SEED,
            MPL_TOKEN_METADATA.to_bytes().as_slice(),
            mint.to_bytes().as_slice(),
        ],
        &MPL_TOKEN_METADATA,
    )
    .0
}

pub fn get_global_pda() -> Pubkey {
    Pubkey::find_program_address(&[GLOBAL_SEED], &PUMP_DOT_FUN_PROGRAM_ID).0
}

pub fn get_creator_vault_pda(creator: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[CREATOR_VAULT_SEED, creator.to_bytes().as_slice()],
        &PUMP_DOT_FUN_PROGRAM_ID,
    )
    .0
}
