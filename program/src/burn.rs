use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};
pub fn burn(
	program_id: &Pubkey,
	for_burn: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
	account: &AccountPDA<spl_token::state::Account>,
	owner: &AccountInfo,
	wallet: &AccountInfo,
) -> ProgramResult {
    gem.data.assoc_account = None;
    csl_spl_token::src::cpi::burn(for_burn, 1)?;
    Ok(())
}