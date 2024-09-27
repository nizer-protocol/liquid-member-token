use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program::{invoke},
  pubkey::Pubkey,
  system_instruction,
};
use spl_token::instruction::burn;

pub fn process_unwrap(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let user_account = next_account_info(accounts_iter)?;
  let wlmt_mint_account = next_account_info(accounts_iter)?;
  let wlmt_token_account = next_account_info(accounts_iter)?;
  let token_program = next_account_info(accounts_iter)?;

  // Burn wlmt tokens
  let burn_wlmt_ix = burn(
      token_program.key,
      wlmt_token_account.key,
      wlmt_mint_account.key,
      user_account.key,
      &[],
      1_000_000_000, // FIXME: provisional amount
  )?;
  invoke(
      &burn_wlmt_ix,
      &[
          token_program.clone(),
          wlmt_token_account.clone(),
          wlmt_mint_account.clone(),
          user_account.clone(),
      ],
  )?;

  // Transfer LMT from WLMT mint back to user's account
  let transfer_wlmt_to_sol_ix = system_instruction::transfer(
      &wlmt_mint_account.key,
      &user_account.key,
      1_000_000_000, // FIXME: provisional amount
  );
  invoke(
      &transfer_wlmt_to_lmt_ix,
      &[wlmt_mint_account.clone(), user_account.clone()],
  )?;

  msg!("Successfully unwrapped WLMT into LMT.");
  Ok(())
}