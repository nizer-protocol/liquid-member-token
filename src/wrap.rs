use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program::{invoke},
  pubkey::Pubkey,
  system_instruction,
};
use spl_token::instruction::mint_to;

pub fn process_wrap(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let user_account = next_account_info(accounts_iter)?;
  let wlmt_mint_account = next_account_info(accounts_iter)?;
  let wlmt_token_account = next_account_info(accounts_iter)?;
  let token_program = next_account_info(accounts_iter)?;

  // Transfer LMT to WLMT mint
  let transfer_lmt_to_wlmt_ix = system_instruction::transfer(
      &user_account.key,
      &wlmt_mint_account.key,
      1_000_000_000, // FIXME: provisional amount
  );
  invoke(
      &transfer_lmt_to_wlmt_ix,
      &[user_account.clone(), wlmt_mint_account.clone()],
  )?;

  // Mint wlmt tokens
  let mint_wlmt_ix = mint_to(
      token_program.key,
      wlmt_mint_account.key,
      wlmt_token_account.key,
      user_account.key,
      &[],
      1_000_000_000, // FIXME: provisional amount
  )?;
  invoke(
      &mint_wlmt_ix,
      &[
          token_program.clone(),
          wlmt_mint_account.clone(),
          wlmt_token_account.clone(),
          user_account.clone(),
      ],
  )?;

  msg!("Successfully wrapped LMT into WLMT.");
  Ok(())
}