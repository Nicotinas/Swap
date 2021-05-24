use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum SwapError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,

    /// Invalid Account
    #[error("Amount Overflow")]
    InvalidAccountData,

}


impl From<SwapError> for ProgramError {
    fn from(e: SwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

