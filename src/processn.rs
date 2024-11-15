use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
};

use spl_token::{
    id as spl_token_program_id, instruction::transfer as token_transfer,
    solana_program::pubkey::Pubkey,
};

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

    let transfer_ix = spl_token::instruction::transfer(
        &token_program_id,    // Program ID of the SPL token
        &source_pubkey,       // Source account (from where tokens will be transferred)
        &destination_pubkey,  // Destination account (to where tokens will be transferred)
        &authority_pubkey,    // Authority (who is allowed to make the transfer)
        &[&authority_pubkey], // Signers (an array of references to the signing public keys)
        amount,               // Amount to transfer (of type u64)
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

    let transfer_ix: spl_token::solana_program::instruction::Instruction =
        spl_token::instruction::transfer(
            &spl_token_program_id(),
            token_mint_account.key,
            seller_token_account.key,
            program_id,
            &[],
            amount,
        )?;

    // Invoke the transfer instruction
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
