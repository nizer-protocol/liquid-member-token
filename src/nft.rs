pub fn mint_and_transfer_nft(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  mint_data: &[u8],
) -> ProgramResult {
  let account_iter = &mut accounts.iter();
  
  // user account
  let user_token_account = next_account_info(account_iter)?;
  let mint_account = next_account_info(account_iter)?;
  let nft_owner_account = next_account_info(account_iter)?;

  let user_balance = get_token_balance(user_token_account)?;

  // mint NFTs based on the number of tokens held
  if user_balance >= REQUIRED_TOKENS {
      mint_nft(mint_account, nft_owner_account, mint_data)?;
      msg!("Successfully minted and transferred the NFT!");
  } else {
      msg!("Insufficient tokens to mint NFT.");
  }
  Ok(())
}

fn get_token_balance(account: &AccountInfo) -> Result<u64, ProgramError> {
  let token_data = Account::unpack(&account.data.borrow())?;
  Ok(token_data.amount)
}