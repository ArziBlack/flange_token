use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::{mint_to, transfer as token_transfer};

const AUTHORIZED_SELLERS: &[&str] = &[
    "FGcSKtGsjLcAqMpnbm7kQbgpBXGEW6bQTXmFntuREjXb",
    "DDRCQBWg58zD67b5moX7Vqdhk4r65jsm3mVtZr75eFmC",
];

fn is_authorized_seller(seller: &Pubkey) -> bool {
    AUTHORIZED_SELLERS
        .iter()
        .any(|&auth| match auth.parse::<Pubkey>() {
            Ok(auth_pubkey) => *seller == auth_pubkey,
            Err(_) => false,
        })
}

pub fn process_buy(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let buyer_token_account = next_account_info(account_info_iter)?;
    let token_mint_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    msg!("Buying {} FLG tokens", amount);

    let transfer_ix = token_transfer(
        &spl_token::id(),
        token_mint_account.key,
        buyer_token_account.key,
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
        seller_token_account.key,
        token_mint_account.key,
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

pub fn process_mint(_program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_info_iter)?;
    let destination_token_account = next_account_info(account_info_iter)?;
    let mint_authority_account = next_account_info(account_info_iter)?;
    let token_program_account = next_account_info(account_info_iter)?;

    if *mint_authority_account.key != *mint_account.key {
        msg!(
            "Unauthorized minting attempt by: {:?}",
            mint_authority_account.key
        );
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Minting {} FLG tokens to destination account", amount);

    let mint_to_ix = mint_to(
        &spl_token::id(),
        mint_account.key,
        destination_token_account.key,
        mint_authority_account.key,
        &[],
        amount,
    )?;

    invoke(
        &mint_to_ix,
        &[
            mint_account.clone(),
            destination_token_account.clone(),
            mint_authority_account.clone(),
            token_program_account.clone(),
        ],
    )?;

    Ok(())
}
