use mpl_token_metadata::{
    instructions::{CreateV1, CreateV1InstructionArgs},
    types::TokenStandard,
};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, system_program, sysvar,
};

/// Creates a metadata account for a fungible token.
///
/// # Arguments
/// * `program_id` - The program ID of the executing program.
/// * `accounts` - The list of account infos required by the instruction.
/// * `metadata` - The metadata account Pubkey.
/// * `mint_pubkey` - The mint account Pubkey.
/// * `payer_pubkey` - The payer Pubkey (who pays for the creation of accounts).
pub fn create_token_metadata_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    metadata: Pubkey,
    mint_pubkey: Pubkey,
    payer_pubkey: Pubkey,
) -> Result<(), ProgramError> {
    let _ = program_id;
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

    // Validate account references (basic checks)
    let metadata_account = accounts.iter().find(|account| account.key == &metadata);
    let mint_account = accounts.iter().find(|account| account.key == &mint_pubkey);
    let payer_account = accounts.iter().find(|account| account.key == &payer_pubkey);

    if metadata_account.is_none() || mint_account.is_none() || payer_account.is_none() {
        return Err(solana_program::program_error::ProgramError::InvalidAccountData);
    }

    // Instruction accounts for creating the metadata
    let create_accounts = CreateV1 {
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
    create_accounts.instruction(args);
    Ok(())
}
