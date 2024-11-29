pub mod freeze;
// pub mod metadata;
pub mod process;
// pub mod utils;

use borsh::{BorshDeserialize, BorshSerialize};
use freeze::freeze_wallet;
// use metadata::{process_create_metadata, process_update_metadata};
// use mpl_token_metadata::state::DataV2;
use process::{process_buy, process_mint, process_sell};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, hash::Hash,
    program_error::ProgramError, pubkey::Pubkey,
};
// use utils::MetadataInfo;

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum FlangeInstruction {
    Buy { amount: u64 },
    Sell { amount: u64 },
    Mint { amount: u64 },
    FreezeWallet,
    // CreateMetaDataV3 {
    //     metadata_info: MetadataInfo,
    //     seller_fee_basis_points: Option<u16>,
    // },
    // UpdateMetaDataV2 {
    //     metadata_key: Pubkey,
    //     update_authority_info: &AccountInfo,
    //     new_update_authority: Option<Pubkey>,
    //     data: Option<DataV2>,
    //     primary_sale_happened: Option<bool>,
    //     is_mutable: Option<bool>,
    // },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ManageMetadataArgs {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub seller_fee_basis_points: Option<u16>,
    pub is_create: bool,
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
        // FlangeInstruction::CreateMetaDataV3 {
        //     metadata_info,
        //     seller_fee_basis_points,
        // } => process_create_metadata(
        //     program_id,
        //     accounts,
        //     metadata_info,
        //     creators,
        //     seller_fee_basis_points,
        // ),
        // FlangeInstruction::UpdateMetaDataV2 {
        //     metadata_key,
        //     update_authority_info,
        //     new_update_authority,
        //     data,
        //     primary_sale_happened,
        //     is_mutable,
        // } => process_update_metadata(
        //     program_id,
        //     accounts,
        //     metadata_key,
        //     update_authority_info,
        //     new_update_authority,
        //     data,
        //     primary_sale_happened,
        //     is_mutable,
        // ),
    }
}
