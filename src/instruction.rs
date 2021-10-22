// program API, (de)serializing instruction data

use crate::error::MailError::InvalidInstruction;
use crate::state::Mail;
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum MailInstruction {
    /// Initialize a new account
    ///
    /// Accounts expected
    ///
    /// 1. `[writable]` The AccountInfo of the account to be initialized
    InitAccount,
    /// Send a mail to an account.
    ///
    /// Accounts expected:
    ///
    /// 1. `[writable]` The AccountInfo of the sender
    /// 2. `[writable]` The AccountInfo of the receiver
    SendMail { mail: Mail },
}

impl MailInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitAccount,
            1 => Self::SendMail {
                mail: Mail::try_from_slice(&rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
