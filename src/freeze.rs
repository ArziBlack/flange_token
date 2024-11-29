use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{freeze_account, transfer_checked},
    state::{Account, Mint},
};

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
    let _ = program_id;
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

pub fn transfer_token(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);
    if expected_authority != *authority_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // unused variable
    let _source_account = Account::unpack(&source_info.try_borrow_data()?)?;

    let mint = Mint::unpack(&mint_info.try_borrow_data()?)?;
    let decimals = mint.decimals;

    msg!("Attempting to transfer {} tokens", amount);
    invoke_signed(
        &transfer_checked(
            token_program_info.key,
            source_info.key,
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[], // no multisig allowed
            amount,
            decimals,
        )
        .unwrap(),
        &[
            source_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_program_info.clone(), // not required, but better for clarity
        ],
        &[&[b"authority", &[bump_seed]]],
    )
}
