use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,    
    pubkey::Pubkey,
    program_pack::Pack,
};
use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use serde_derive::{Deserialize, Serialize};
use crate::error::SwapError::InvalidInstruction;
use crate::state::AccountProfile;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SwapInstruction {

/// This will store inforamtion of individual account 
/// Instruction serialisation and deserialisation.
/// Accounts Expected:
/// 0. [signer] The account of the person who is creating an account.

SetUserProfile {
    name: String,
    age: u8,
    dob: String,
}

}

impl SwapInstruction {

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        
        msg!("input {:?}", input);

        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        let ab = AccountProfile::try_from_slice(&input);

        msg!("ab = {:?}", ab);
        msg!("tag={:?}", tag);
        msg!("rest={:?}", rest);

        let dc = AccountProfile::try_from_slice(&rest).unwrap();
        msg!("dc = {:?}", dc);


        // Ok({Self::SetUserProfile {
        //         name: Self::unpack_name(rest)?,
        //         age: Self::unpack_age(rest)?,
        //         dob: Self::unpack_dob(rest)?,
        //     },
        //     _ => return Err(InvalidInstruction.into())
        // }
        // )

        Ok(Self::SetUserProfile {name: String::from("Shivin"), age: 22, dob: String::from("15/06/1995")})

    }

    fn unpack_name(input: &[u8]) -> Result<String, ProgramError> {
        let name = "Shivin";
        msg!("input = {:?}", input);

        Ok(name.to_string())
    }
    fn unpack_age(input: &[u8]) -> Result<u8, ProgramError> {
        let name = 22;
        msg!("input = {:?}", input);

        Ok(name)
    }
    fn unpack_dob(input: &[u8]) -> Result<String, ProgramError> {
        let name = "Shivin";
        msg!("input = {:?}", input);
        Ok(name.to_string())
    }

}
