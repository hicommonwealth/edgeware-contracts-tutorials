#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ballot {
    // Structure to store Proposal information
    struct Proposal {
        name: String,
        vote_count: i32, 
    }

    // Structure to store Proposal information
    pub struct Voter {
        weight: i32,
        voted: bool,
        delegate: Option<AccountId>, 
        vote: Option<i32>, 
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Ballot {
        chair_person: AccountId,
    }

    impl Ballot {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self { 
                chair_person:owner,
              }
        }


        #[ink(message)]
        pub fn get_chairperson(&self) -> AccountId {
            self.chair_person
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        
        // Alias `ink_lang` so we can use `ink::test`.
        use ink_lang as ink;
        
        #[ink::test]
        fn new_works() {
            let ballot = Ballot::new();
            assert_eq!(ballot.get_chairperson(),AccountId::from([0x1; 32]));
        }
    }
}
