pub mod freeze;
pub mod metadata;
pub mod process;

use borsh::{BorshDeserialize, BorshSerialize};
use freeze::freeze_wallet;
use metadata::create_token_metadata_instruction;
use process::{process_buy, process_mint, process_sell};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FlangeInstruction {
    Buy {
        amount: u64,
    },
    Sell {
        amount: u64,
    },
    Mint {
        amount: u64,
    },
    FreezeWallet,
    CreateMetaData {
        metadata: Pubkey,
        mint_pubkey: Pubkey,
        payer_pubkey: Pubkey,
    },
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = FlangeInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        FlangeInstruction::Buy { amount } => process_buy(program_id, accounts, amount),
        FlangeInstruction::Sell { amount } => process_sell(program_id, accounts, amount),
        FlangeInstruction::Mint { amount } => process_mint(program_id, accounts, amount),
        FlangeInstruction::FreezeWallet => freeze_wallet(program_id, accounts),
        FlangeInstruction::CreateMetaData {
            metadata,
            mint_pubkey,
            payer_pubkey,
        } => create_token_metadata_instruction(
            program_id,
            accounts,
            metadata,
            mint_pubkey,
            payer_pubkey,
        ),
    }
}
