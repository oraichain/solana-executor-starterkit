#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

pub use solana_program;
solana_program::declare_id!("CETAPVg8wMdFKUeEsoXQKpfsj4fQLJS3J6UdRLRj66GC");

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        entrypoint::process_instruction, instruction::HelloWorldInstruction, state::GreetingAccount,
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{account_info::AccountInfo, clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        // create instruction
        let instruction_data = HelloWorldInstruction::Increment {}.try_to_vec().unwrap();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        // instruction as borsh format
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        // this time data is increased to 2
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
