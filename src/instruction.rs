use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::BetError::InvalidInstruction;

pub enum BetInstruction {
    /// 0. `[signer, writable] The sender account
    /// 1. `[]` The bet program account, it needs to receive SOL
    /// 2. `[]` The token program
    /// ?? The rent sysvar
    Stake {
        amount: u64,
        // side : u8
        // the input needs to have a fixed size(but it will be because it uses hash)
        // any other good data type for binary data ? bool ?
    },

    /// 0. `[signer] The receiver account
    /// 1. `[writable]` The bet program account, it needs to send SOL. Q) Need to be writable ?
    /// 1. `[]` The token program
    /// 2. `[]` The PDA account
    ClaimReward {
        amount: u64,
    }
}

impl BetInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Stake {
                amount: Self::unpack_amount(rest)?
            },
            1 => Self::ClaimReward {
                amount: Self::unpack_amount(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
