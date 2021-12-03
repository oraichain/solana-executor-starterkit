use crate::instruction::HelloWorldInstruction;
use crate::processor::process_increment;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// override process_instruction entry
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Entrypoint");
    let instruction = HelloWorldInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        HelloWorldInstruction::Increment {} => {
            msg!("HelloWorldInstruction::Increment");
            process_increment(program_id, accounts)
        }
    }
}
