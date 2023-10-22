use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};

pub fn transfer(
	program_id: &Pubkey,
	for_create: &[&AccountInfo],
	for_transfer_checked: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	source: &AccountInfo,
	destination: &AccountInfo,
	authority: &AccountInfo,
) -> ProgramResult {
    gem.data.assoc_account = Some(*destination.key);

    // Create the ATA account for new owner if it hasn't been created
    if assoc_token_account.lamports() == 0 {
        csl_spl_assoc_token::src::cpi::create(for_create)?;
    }

    csl_spl_token::src::cpi::transfer_checked(for_transfer_checked, 1, 0)?;
    Ok(())
}