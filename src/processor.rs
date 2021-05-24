use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::error::SwapError::InvalidInstruction;
use crate::instruction::SwapInstruction;
use crate::state::AccountProfile;


pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        msg!("instruction_data = {:?}", instruction_data);

        let instruction = SwapInstruction::unpack(instruction_data)?;

        // msg!("instruction = {:?}", instruction);

        match instruction {
            SwapInstruction::SetUserProfile { name, age, dob } => {
                msg!("name = {:?}", name);
                msg!("age = {:?}", age);
                msg!("dob = {:?}", dob);

                Self::SetUserProfile(accounts, name, age, dob, program_id)
            }
        }

    }

    fn SetUserProfile(
        accounts: &[AccountInfo],
        name: String,
        age: u8,
        dob: String,
        program_id: &Pubkey
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        msg!("account_info_iter = {:?}", account_info_iter);

        let account = next_account_info(account_info_iter)?;

        msg!("account = {:?}", account);

        let decoded = AccountProfile::unpack(&account.try_borrow_data()?);

        let mut out = match decoded {
            Ok(u) => u,
            Err(_) => AccountProfile::default(),
        };
        msg!("Processor dob = {:?}", dob);
        msg!("Processor age = {:?}", age);
        msg!("Processor dob  = {:?}", name);

        // if let Some(i) = dob {
        //     out.dob = i;
        // }
        // if let Some(i) = name {
        //     out.name = Some(i);
        // }
        // if let Some(i) = age {
        //     out.age = Some(i);
        // }

        let mut user_profile_data = account.try_borrow_mut_data()?;
        out.pack(&mut user_profile_data);
        Ok(())
    }
}

