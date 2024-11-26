use metaplex_token_metadata::{
    instruction::{create_metadata_accounts_v2, update_metadata_accounts_v2},
    state::{Creator, Metadata},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
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
