use crate::{
    constants::{
        ASSOCIATED_TOKEN_PROGRAM, EVENT_AUTHORITY, MPL_TOKEN_METADATA, RENT_SYSVAR, SYSTEM_PROGRAM,
        TOKEN_PROGRAM,
    },
    pda::{get_associated_bonding_curve, get_bonding_curve_pda, get_global_pda, get_metadata_pda},
    PUMP_DOT_FUN_DEVENT_PROGRAM_ID,
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
    let global_pda = get_global_pda();

    let accounts = vec![
        AccountMeta::new(accounts.mint, true),
        AccountMeta::new(bonding_curve, false),
        AccountMeta::new(associated_bonding_curve, false),
        AccountMeta::new(global_pda, false),
        AccountMeta::new_readonly(MPL_TOKEN_METADATA, false),
        AccountMeta::new(metadata_pda, false),
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

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_create_ix() {
        // Mock data for CreateAccounts
        let accounts = CreateAccounts {
            mint: Pubkey::new_unique(),
            user: Pubkey::new_unique(),
        };

        // Mock data for CreateArgs
        let args = CreateArgs {
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            uri: "http://test.uri".to_string(),
            creator: Pubkey::new_unique(),
        };

        let program_id = Pubkey::new_unique(); // Dummy program ID for testing

        let instruction = create_ix(&program_id, accounts, args);

        // Assertions
        assert_eq!(instruction.program_id, program_id);
        // Add more assertions here to check accounts and data
        // For example:
        // assert_eq!(instruction.accounts.len(), 13); // Check the number of accounts
        // assert!(instruction.data.starts_with(&[24, 30, 200, 40, 5, 28, 7, 119])); // Check discriminator
    }
}
