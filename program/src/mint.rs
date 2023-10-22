use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};
pub fn mint(
	program_id: &Pubkey,
	for_initialize_mint_2: &[&AccountInfo],
	for_create: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	for_set_authority: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	owner: &AccountInfo,
	color: String,
	rarity: String,
	short_description: String,
	edition: u64,
	max_supply: u64,
) -> ProgramResult {
    gem.data.color = color;
    gem.data.rarity = rarity;
    gem.data.short_description = short_description;
    gem.data.mint = *mint.info.key;
    gem.data.assoc_account = Some(*assoc_token_account.key);

    csl_spl_token::src::cpi::initialize_mint_2(for_initialize_mint_2, 0, *wallet.key, None)?;
    csl_spl_assoc_token::src::cpi::create(for_create)?;
    csl_spl_token::src::cpi::mint_to(for_mint_to, 1)?;
    csl_spl_token::src::cpi::set_authority(for_set_authority, 0, None)?;
    Ok(())
}