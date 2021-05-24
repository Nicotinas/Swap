use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::SwapError::InvalidAccountData;
use solana_program::program_error::ProgramError;
// use solana_sdk::{
//     clock::UnixTimestamp,
//     program_error::ProgramError,
//     pubkey::{Pubkey, PubkeyError, MAX_SEED_LEN},
// };
use std::mem;



pub type PublicKey = [u8; 32];

// pub trait Serdes: Sized + BorshSerialize + BorshDeserialize {
//     fn pack(&self, dst: &mut [u8]) {
//         let encoded = self.try_to_vec().unwrap();
//         dst[..encoded.len()].copy_from_slice(&encoded);
//     }
//     fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
//         Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
//     }
// }

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct AccountProfile {
    pub name: Option<String>,
    pub age: u8,
    pub dob: Option<String>,
}
impl AccountProfile {

    pub fn pack(&self, dst: &mut [u8]) {
        let encoded = self.try_to_vec().unwrap();
        dst[..encoded.len()].copy_from_slice(&encoded);
    }

    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
    }
    // pub fn create_with_seed(user_pk: &Pubkey, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
        // Pubkey::create_with_seed(&user_pk, AccountProfile::SEED, &program_id)
    // }
}
impl Default for AccountProfile {
    fn default() -> Self {
        Self {
            name: None,
            age: 0,
            dob: None,
        }
    }
}



// impl Serdes for AccountProfile {}
