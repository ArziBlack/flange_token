use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::ID as SPL_TOKEN_PROGRAM_ID;

const AUTHORIZED_SELLERS: &[&str] = &[
    "FGcSKtGsjLcAqMpnbm7kQbgpBXGEW6bQTXmFntuREjXb",
    "SellerPubkey2XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
];

/// Checks if a given seller is authorized to sell tokens.
fn is_authorized_seller(seller: &Pubkey) -> bool {
    AUTHORIZED_SELLERS
        .iter()
        .any(|&auth| auth.parse::<Pubkey>().ok() == Some(*seller))
}

/// Processes a token purchase transaction.
pub fn process_buy(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let buyer_token_account = next_account_info(account_info_iter)?;
    let token_mint_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    msg!("Buying {} FLG tokens", amount);

    let transfer_ix = token_transfer(
        &spl_token::id(),
        &token_mint_account.key,
        &buyer_token_account.key,
        program_id,
        &[],
        amount,
    )?;

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

pub fn process_sell(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let seller_token_account = next_account_info(account_info_iter)?;
    let token_mint_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    if !is_authorized_seller(seller_token_account.key) {
        msg!("Unauthorized seller: {:?}", seller_token_account.key);
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Selling {} FLG tokens", amount);

    let transfer_ix = token_transfer(
        &spl_token::id(),
        &seller_token_account.key,
        &token_mint_account.key,
        program_id,
        &[],
        amount,
    )?;

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
