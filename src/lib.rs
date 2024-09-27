mod wrap;
mod unwrap;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Assume the first byte of `instruction_data` indicates the operation:
    // 0 for wrap, 1 for unwrap
    match instruction_data[0] {
        0 => wrap::process_wrap(program_id, accounts, instruction_data),
        1 => unwrap::process_unwrap(program_id, accounts, instruction_data),
        _ => {
            msg!("Invalid instruction");
            Err(solana_program::program_error::ProgramError::InvalidInstructionData)
        }
    }
}