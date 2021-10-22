// program specific errors

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MailError {
    #[error("Invalid Instruction!")]
    InvalidInstruction,

    #[error("Account is not Writable!")]
    NotWritable,
}

impl From<MailError> for ProgramError {
    fn from(err: MailError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
