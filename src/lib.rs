pub mod process;

use borsh::{BorshDeserialize, BorshSerialize};
use process::{process_buy, process_mint, process_sell};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FlangeInstruction {
    Buy { amount: u64 },
    Sell { amount: u64 },
    Mint { amount: u64 },
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
    }
}
