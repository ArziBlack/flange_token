use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::{instruction::freeze_account, state::Account};

const WHITELIST: &[&str] = &[
    "FGcSKtGsjLcAqMpnbm7kQbgpBXGEW6bQTXmFntuREjXb",
    "DDRCQBWg58zD67b5moX7Vqdhk4r65jsm3mVtZr75eFmC",
];

fn is_whitelisted(pubkey: &Pubkey) -> bool {
    WHITELIST
        .iter()
        .any(|&addr| addr.parse::<Pubkey>().map_or(false, |w| *pubkey == w))
}

pub fn freeze_wallet(program_id: &Pubkey, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();

    let wallet_to_free = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let token_mint = next_account_info(account_info_iter)?;
    let freeze_authority = next_account_info(account_info_iter)?;

    if !is_whitelisted(freeze_authority.key) {
        msg!("Unauthorized freeze authority: {:?}", freeze_authority.key);
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Freezing wallet: {:?}", wallet_to_free.key);

    let freeze_ix = freeze_account(
        &spl_token::id(),
        token_account.key,
        token_mint.key,
        freeze_authority.key,
        &[],
    )?;

    solana_program::program::invoke(
        &freeze_ix,
        &[
            token_account.clone(),
            token_mint.clone(),
            freeze_authority.clone(),
        ],
    )?;

    Ok(())
}
