use mpl_token_metadata::{
    instructions::{CreateV1, CreateV1InstructionArgs},
    types::TokenStandard,
};
use solana_program::{pubkey::Pubkey, system_program, sysvar};

/// Creates a metadata account for a fungible token.
///
/// # Arguments
/// * `metadata` - The metadata account Pubkey.
/// * `mint_pubkey` - The mint account Pubkey.
/// * `payer_pubkey` - The payer Pubkey (who pays for the creation of accounts).
pub fn create_token_metadata_instruction(
    metadata: Pubkey,
    mint_pubkey: Pubkey,
    payer_pubkey: Pubkey,
) -> solana_program::instruction::Instruction {
    // Instruction arguments for creating metadata for a fungible token
    let args = CreateV1InstructionArgs {
        name: String::from("Flange Token"), // Token name
        symbol: String::from("FLGT"),       // Token symbol
        uri: String::from("https://example.com/metadata.json"), // Metadata URI
        seller_fee_basis_points: 0,         // No royalties for fungible tokens
        primary_sale_happened: false,       // No initial sale
        is_mutable: true,                   // Allow updates to metadata
        token_standard: TokenStandard::Fungible, // Set token standard to fungible
        collection: None,                   // Not applicable for fungible tokens
        uses: None,                         // Not applicable for fungible tokens
        collection_details: None,           // Not applicable for fungible tokens
        creators: None,                     // Optional creators (e.g., for attribution)
        rule_set: None,                     // No programmable rules
        decimals: Some(6),                  // Example: 6 decimals for fungible tokens
        print_supply: None,                 // Not applicable for fungible tokens
    };

    // Instruction accounts for creating the metadata
    let create_accounts: CreateV1 = CreateV1 {
        metadata,
        master_edition: None, // Master edition is not required for fungible tokens
        mint: (mint_pubkey, true), // Mint account, must be a signer
        authority: payer_pubkey, // Token authority
        payer: payer_pubkey,  // Payer for account creation
        update_authority: (payer_pubkey, true), // Update authority, must be a signer
        system_program: system_program::ID, // System program
        sysvar_instructions: sysvar::instructions::ID, // Sysvar instructions account
        spl_token_program: spl_token::ID, // SPL token program
    };

    // Generate and return the instruction
    create_accounts.instruction(args)
}

// Function to create the metadata for a fungible token using CreateMetadataAccountV3
// pub fn create_metadata_account_v3_instruction(
//     metadata: Pubkey,
//     mint_pubkey: Pubkey,
//     mint_authority: Pubkey,
//     payer_pubkey: Pubkey,
//     update_authority_pubkey: Pubkey,
//     system_program_id: Pubkey,
//     rent_account: Option<Pubkey>,
//     token_name: String,
//     token_symbol: String,
//     token_uri: String,
// ) -> Instruction {
//     let _data = DataV2 {
//         name: token_name,
//         symbol: token_symbol,
//         uri: token_uri,
//         seller_fee_basis_points: todo!(),
//         creators: todo!(),
//         collection: todo!(),
//         uses: todo!(),
//     };

//     let _args: CreateMetadataAccountV3InstructionArgs = CreateMetadataAccountV3InstructionArgs {
//         data: _data,
//         is_mutable: true,         // Make the metadata mutable
//         collection_details: None, // Optional: Set this to Some(CollectionDetails) if needed
//     };

//     let _metadata_account = CreateMetadataAccountV3 {
//         metadata,
//         mint: mint_pubkey,
//         mint_authority,
//         payer: payer_pubkey,
//         update_authority: (update_authority_pubkey, true),
//         system_program: todo!(),
//         rent: Some(solana_program::sysvar::rent::ID), // Set update authority and whether it's a signer
//     };

//     // Create the metadata account instruction
//     _metadata_account.instruction(_args)
// }
