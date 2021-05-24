use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
    program_pack::Pack,
    msg,
};

use crate::processor::Processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("program_id = {:?}", program_id);
    msg!("accounts = {:?}", accounts);
    msg!("instruction_data = {:?}", instruction_data);
    Processor::process(program_id, accounts, instruction_data)
}