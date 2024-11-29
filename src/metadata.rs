// use mpl_token_metadata::{
//     instruction::{create_metadata_accounts_v3, update_metadata_accounts_v2},
//     state::{Creator, DataV2, Metadata},
//     ID,
// };

// use solana_program::{
//     account_info::{next_account_info, AccountInfo},
//     msg,
//     program::{invoke, invoke_signed},
//     program_error::ProgramError,
//     pubkey::Pubkey,
//     system_instruction,
// };

// pub struct MetadataInfo {
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
// }

// pub fn process_create_metadata(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     metadata_info: MetadataInfo,
//     creators: Option<Vec<Creator>>,
//     seller_fee_basis_points: u16,
// ) -> Result<(), ProgramError> {
//     let account_info_iter = &mut accounts.iter();

//     let payer = next_account_info(account_info_iter)?;
//     let mint_account = next_account_info(account_info_iter)?;
//     let metadata_account = next_account_info(account_info_iter)?;
//     let token_program = next_account_info(account_info_iter)?;
//     let mint_authority = next_account_info(account_info_iter)?;

//     if !payer.is_signer {
//         return Err(ProgramError::MissingRequiredSignature);
//     }

//     let metadata_account_key = *metadata_account.key;

//     let data = DataV2 {
//         name: metadata_info.name,
//         symbol: metadata_info.symbol,
//         uri: metadata_info.uri,
//         seller_fee_basis_points,
//         creators,
//         collection: todo!(),
//         uses: todo!(),
//     };

//     let create_metadata_ix = create_metadata_accounts_v3(
//         ID,
//         metadata_account_key,
//         *mint_account.key,
//         *mint_authority.key,
//         *payer.key,
//         *payer.key,
//         data.name,
//         data.symbol,
//         data.uri,
//         data.creators,
//         data.seller_fee_basis_points,
//         true,
//         true,
//         None,
//         None,
//         None,
//     );

//     invoke(
//         &create_metadata_ix,
//         &[
//             payer.clone(),
//             mint_account.clone(),
//             metadata_account.clone(),
//             token_program.clone(),
//         ],
//     )?;

//     msg!("Metadata account created successfully using v3.");
//     Ok(())
// }

// pub fn process_update_metadata(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     metadata_key: Pubkey,
//     update_authority_info: &AccountInfo,
//     new_update_authority: Option<Pubkey>,
//     data: Option<DataV2>,
//     primary_sale_happened: Option<bool>,
//     is_mutable: Option<bool>,
// ) -> Result<(), ProgramError> {
//     let account_info_iter = &mut accounts.iter();

//     let metadata_account = next_account_info(account_info_iter)?;
//     let update_authority = next_account_info(account_info_iter)?;

//     if !update_authority.is_signer {
//         return Err(ProgramError::MissingRequiredSignature);
//     }

//     if metadata_account.key != &metadata_key {
//         return Err(ProgramError::InvalidAccountData);
//     }

//     let update_metadata_ix = update_metadata_accounts_v2(
//         mpl_token_metadata::ID,
//         *metadata_account.key,
//         *update_authority.key,
//         new_update_authority,
//         data,
//         primary_sale_happened,
//         is_mutable,
//     );

//     invoke(
//         &update_metadata_ix,
//         &[metadata_account.clone(), update_authority.clone()],
//     )?;

//     msg!("Metadata account updated successfully using update_metadata_accounts_v2.");
//     Ok(())
// }
