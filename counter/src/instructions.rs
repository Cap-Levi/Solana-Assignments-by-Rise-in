use borsh::{BorshDeserialize,BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs{
    pub value: u32,
}
 
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserInput{
    pub value: u32,
 }
 
pub enum CounterInstructions {
    Increment(UserInput),
    Decrement(UserInput),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(UserInput::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UserInput::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),

        })
    }
}