// use metaplex_token_metadata::{
//     instruction::{create_metadata_accounts_v2, update_metadata_accounts_v2},
//     state::{Creator, Metadata},
// };

use mpl_token_metadata::{
    instruction::{create_metadata_accounts_v2, update_metadata_accounts_v2},
    state::{Creator, DataV2, Metadata},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

const METAPLEX_PROGRAM_ID: &str = "metaqbxxUerdq28cL77GtYg2gXcsbhqG4xw9nuwNji6";

pub struct MetadataInfo {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn process_create_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    metadata_info: MetadataInfo,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let metadata_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let metadata_account_key = *metadata_account.key;

    let metadata = Metadata {
        name: metadata_info.name.clone(),
        symbol: metadata_info.symbol.clone(),
        uri: metadata_info.uri.clone(),
        creators: None,
        seller_fee_basis_points: 500,
        update_authority: *payer.key,
        primary_sale_happened: false,
        is_mutable: true,
    };

    let create_metadata_ix = create_metadata_accounts_v2(
        &METAPLEX_PROGRAM_ID.parse().unwrap(),
        metadata_account_key,
        mint_account.key.clone(),
        payer.key.clone(),
        payer.key.clone(),
        payer.key.clone(),
        metadata.name,
        metadata.symbol,
        metadata.uri,
        None,
        500,
        true,
    );

    invoke(
        &create_metadata_ix,
        &[
            payer.clone(),
            mint_account.clone(),
            metadata_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Metadata account created successfully.");
    Ok(())
}

pub fn update_metadata_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    metadata_info: MetadataInfo,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let metadata_account = next_account_info(account_info_iter)?;

    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let metadata_account_key = *metadata_account.key;

    let update_metadata_ix = update_metadata_accounts_v2(
        &METAPLEX_PROGRAM_ID.parse().unwrap(),
        metadata_account_key,
        payer.key.clone(),
        Some(payer.key.clone()),
        Some(metadata_info.name),
        Some(metadata_info.symbol),
        Some(metadata_info.uri),
        None,
        None,
        true,
    );

    invoke(
        &update_metadata_ix,
        &[payer.clone(), metadata_account.clone()],
    )?;

    msg!("Metadata account updated successfully.");
    Ok(())
}

pub fn create_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let metadata_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let mint_authority = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_account = next_account_info(account_info_iter)?;

    let metadata_instruction = create_metadata_accounts_v2(
        mpl_token_metadata::id(), // Metadata program ID
        *metadata_account.key,    // Metadata account
        *mint_account.key,        // Mint address
        *mint_authority.key,      // Mint authority
        *payer.key,               // Payer
        *payer.key,               // Update authority
        name,                     // Name of the token
        symbol,                   // Symbol of the token
        uri,                      // URI pointing to metadata JSON
        creators,                 // Optional creators
        seller_fee_basis_points,  // Royalties in basis points
        true,                     // Update authority is signer
        true,                     // Is mutable
    );

    invoke(
        &metadata_instruction,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            system_program.clone(),
            rent_account.clone(),
        ],
    )?;

    Ok(())
}

pub fn update_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: Option<u16>,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let metadata_account = next_account_info(account_info_iter)?;
    let update_authority = next_account_info(account_info_iter)?;

    // Load existing metadata
    let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data.borrow())?;

    let updated_data = DataV2 {
        name: name.unwrap_or(metadata.data.name),
        symbol: symbol.unwrap_or(metadata.data.symbol),
        uri: uri.unwrap_or(metadata.data.uri),
        seller_fee_basis_points: seller_fee_basis_points
            .unwrap_or(metadata.data.seller_fee_basis_points),
        creators: creators.or(metadata.data.creators),
    };

    let update_metadata_ix = update_metadata_accounts_v2(
        mpl_token_metadata::id(),
        *metadata_account.key,
        *update_authority.key,
        None, // No new update authority
        Some(updated_data),
        None, // No primary sale state change
    );

    invoke(
        &update_metadata_ix,
        &[metadata_account.clone(), update_authority.clone()],
    )?;

    Ok(())
}

// combined instruction  for create and update metadata
pub fn manage_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: Option<u16>,
    is_create: bool,
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let metadata_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let payer_account = next_account_info(account_info_iter)?;
    let update_authority_account = next_account_info(account_info_iter)?;

    if is_create {
        // Creating metadata
        let data = DataV2 {
            name: name.unwrap_or_default(),
            symbol: symbol.unwrap_or_default(),
            uri: uri.unwrap_or_default(),
            seller_fee_basis_points: seller_fee_basis_points.unwrap_or(0),
            creators,
        };

        let create_metadata_ix = create_metadata_accounts_v2(
            mpl_token_metadata::id(),
            *metadata_account.key,
            *mint_account.key,
            *update_authority_account.key,
            *payer_account.key,
            *update_authority_account.key,
            data,
            true,
            true,
        );

        invoke(
            &create_metadata_ix,
            &[
                metadata_account.clone(),
                mint_account.clone(),
                payer_account.clone(),
                update_authority_account.clone(),
            ],
        )?;
    } else {
        // Updating metadata
        let metadata: Metadata = try_from_slice_unchecked(&metadata_account.data.borrow())?;

        let updated_data = DataV2 {
            name: name.unwrap_or(metadata.data.name),
            symbol: symbol.unwrap_or(metadata.data.symbol),
            uri: uri.unwrap_or(metadata.data.uri),
            seller_fee_basis_points: seller_fee_basis_points
                .unwrap_or(metadata.data.seller_fee_basis_points),
            creators: creators.or(metadata.data.creators),
        };

        let update_metadata_ix = update_metadata_accounts_v2(
            mpl_token_metadata::id(),
            *metadata_account.key,
            *update_authority_account.key,
            None, // No new update authority
            Some(updated_data),
            None, // No primary sale state change
        );

        invoke(
            &update_metadata_ix,
            &[metadata_account.clone(), update_authority_account.clone()],
        )?;
    }

    Ok(())
}
