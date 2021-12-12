use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
};

use crate::{instruction::BetInstruction, error::BetError};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = BetInstruction::unpack(instruction_data)?;

        match instruction {
            BetInstruction::Stake { amount } => {
                msg!("Instruction: Stake");
                Self::process_stake(accounts, amount, program_id)
            },
            BetInstruction::ClaimReward { amount } => {
                msg!("Instruction: Claim reward");
                Self::process_claim_reward(accounts, amount, program_id)
            }
        }
    }

    fn process_stake (
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let sender = next_account_info(account_info_iter)?;
        if !sender.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let bet_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let transfer_to_program_account_ix = spl_token::instruction::transfer(
            token_program.key,
            sender.key,
            bet_account.key,
            sender.key,
            &[&sender.key],
            amount,
        )?;
        msg!("Calling the token program to transfer SOL to the program account");
        invoke(
            &transfer_to_program_account_ix,
            &[
                sender.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_claim_reward (
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let receiver = next_account_info(account_info_iter)?;

        let bet_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        let (pda, nonce) = Pubkey::find_program_address(&[b"minority"], program_id); // PDA ...
        let pda_account = next_account_info(account_info_iter)?;

        let transfer_to_receiver_ix = spl_token::instruction::transfer(
            token_program.key,
            pda_account.key,
            receiver.key,
            &pda,
            &[&pda],
            amount,
        )?;

        msg!("Calling the token program to transfer SOL to the receiver");
        invoke_signed(
            &transfer_to_receiver_ix,
            &[
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"minority"[..], &[nonce]]],
        )?;

        Ok(())
    }
}