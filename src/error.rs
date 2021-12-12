use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum BetError {
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<BetError> for ProgramError {
    fn from(e: BetError) -> Self {
        ProgramError::Custom(e as u32)
    }
}