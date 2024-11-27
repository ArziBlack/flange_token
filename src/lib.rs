pub mod freeze;
pub mod metadata;
pub mod process;

use std::env::args;

use freeze::freeze_wallet;
use metadata::{
    create_metadata, process_create_metadata, process_update_metadata, update_metadata,
};
use process::{process_buy, process_mint, process_sell};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

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
    CreateMetadata {
        name: String,
        symbol: String,
        uri: String,
    },
    CreateMetadataV2 {
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
    },
    UpdateMetadata {
        uri: String,
    },
    UpdateMetadataV2 {
        name: Option<String>,
        symbol: Option<String>,
        uri: Option<String>,
        seller_fee_basis_points: Option<u16>,
    },
    ManageMetadata(ManageMetadataArgs),
    FreezeWallet,
}

pub struct ManageMetadataArgs {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub seller_fee_basis_points: Option<u16>,
    pub is_create: bool,
}

pub fn my_try_from_slice_unchecked<T: borsh::BorshDeserialize>(
    data: &[u8],
) -> Result<T, ProgramError> {
    let mut data_mut = data;
    match T::deserialize(&mut data_mut) {
        Ok(result) => Ok(result),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: FlangeInstruction =
        FlangeInstruction::my_try_from_slice_unchecked(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        FlangeInstruction::Buy { amount } => process_buy(program_id, accounts, amount),
        FlangeInstruction::Sell { amount } => process_sell(program_id, accounts, amount),
        FlangeInstruction::Mint { amount } => process_mint(program_id, accounts, amount),
        FlangeInstruction::CreateMetadata { name, symbol, uri } => {
            process_create_metadata(program_id, accounts, metadata_info)
        }
        FlangeInstruction::CreateMetadataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points,
        } => create_metadata(
            program_id,
            accounts,
            name,
            symbol,
            uri,
            None,
            seller_fee_basis_points,
        ),
        FlangeInstruction::UpdateMetadataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points,
        } => update_metadata(
            program_id,
            accounts,
            name,
            symbol,
            uri,
            None,
            seller_fee_basis_points,
        ),
        FlangeInstruction::UpdateMetadata { uri } => {
            process_update_metadata(program_id, accounts, uri)
        }
        FlangeInstruction::ManageMetadata => manage_metadata(
            program_id,
            accounts,
            name,
            symbol,
            uri,
            None,
            seller_fee_basis_points,
            is_create,
        ),
        FlangeInstruction::FreezeWallet => freeze_wallet(program_id, accounts),
    }
}
