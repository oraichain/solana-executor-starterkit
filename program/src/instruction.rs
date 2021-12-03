// use crate::state::GreetingAccount;

use borsh::{BorshDeserialize, BorshSerialize};

/// Instructions supported by the generic Name Registry program
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum HelloWorldInstruction {
    Increment {},
}
