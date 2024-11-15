use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
};

use spl_token::{instruction::transfer as token_transfer, solana_program::pubkey::Pubkey};

const AUTHORIZED_SELLERS: &[&str] = &[
    "FGcSKtGsjLcAqMpnbm7kQbgpBXGEW6bQTXmFntuREjXb",
    "SellerPubkey2XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
];

/// Check if the seller is authorized
fn is_authorized_seller(seller: &Pubkey) -> bool {
    AUTHORIZED_SELLERS
        .iter()
        .any(|&auth| auth.parse::<Pubkey>().ok() == Some(*seller))
}

/// Processes a token purchase transaction
pub fn process_buy(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let buyer_token_account = next_account_info(account_info_iter)?;
    let token_mint_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    msg!("Buying {} FLG tokens", amount);

    let transfer_ix = token_transfer(
        &SPL_TOKEN_PROGRAM_ID,
        &token_mint_account.key,
        &buyer_token_account.key,
        program_id,
        &[],
        amount,
    )?;

    // Invoke token transfer instruction
    invoke(
        &transfer_ix,
        &[
            token_mint_account.clone(),
            buyer_token_account.clone(),
            token_program_account.clone(),
        ],
    )?;

    Ok(())
}

/// Processes a token sale transaction
pub fn process_sell(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let seller_token_account = next_account_info(account_info_iter)?;
    let token_mint_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    // Authorization check
    if !is_authorized_seller(seller_token_account.key) {
        msg!("Unauthorized seller: {:?}", seller_token_account.key);
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Selling {} FLG tokens", amount);

    // Build token transfer instruction
    let transfer_ix: Result<
        spl_token::solana_program::instruction::Instruction,
        spl_token::solana_program::program_error::ProgramError,
    > = token_transfer(
        &SPL_TOKEN_PROGRAM_ID,
        &seller_token_account.key,
        &token_mint_account.key,
        program_id,
        &[],
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            seller_token_account.clone(),
            token_mint_account.clone(),
            token_program_account.clone(),
        ],
    )?;

    Ok(())
}
