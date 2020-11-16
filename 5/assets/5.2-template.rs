#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]    
mod ballot {
    use ink_storage::collections::HashMap;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{PackedLayout, SpreadLayout};

    // Structure to store Proposal information
    #[derive(Clone, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,scale_info::TypeInfo)]
    struct Proposal {
        name: String,
        vote_count: u32, 
    }

    // Structure to store Proposal information
    #[derive(Clone, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,scale_info::TypeInfo)]
    pub struct Voter {
        weight: u32,
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
        //  ACTION: create a voters hash map with account id as key and Voter as value
        voters: 

        //  ACTION: create a proposals vector
        proposals: 
    }

    impl Ballot {
        #[ink(constructor)]
        pub fn new() -> Self {
            // get chair person address
            let chair_person =  Self::env().caller();
    
            //  ACTION: create empty proposals and voters variables
            //          * let proposals = 
            //          * let mut voters =

            // ACTION: add chair persons voter object in voters hash map
            // HINT: Use hashmap.insert(key,value)

            Self {
                chair_person,
                voters,
                proposals,
            }
        }

        // return chair person id
        #[ink(message)]
        pub fn get_chairperson(&self) -> AccountId {
            self.chair_person
        }

        // return the provided voter object
        pub fn get_voter(&self, voter_id: AccountId) -> Option<&Voter>{
        }

        // return the count of voters
        pub fn get_voter_count(&self) -> usize{
        }


        /// the function adds the provided voter id into possible
        /// list of voters. By default the voter has no voting right,
        /// the contract owner must approve the voter before he can cast a vote
        #[ink(message)]
        pub fn add_voter(&mut self, voter_id: AccountId) -> bool{
            // ACTION: check if voter already exits, if yes return false       
            //      * if not exists, create an entry in hash map 
            //      * with default weight set to 0 and voted to false
            //      * and return true
            
            // HINT: use hashmap.get() to get voter
            //        and use options.some() to check if voter exists
        }

        /// given an index returns the name of the proposal at that index
        pub fn get_proposal_name_at_index(&self, index:usize) -> &String {
        }

        /// returns the number of proposals in ballet
        pub fn get_proposal_count(&self) -> usize {
        }


        /// adds the given proposal name in ballet
        pub fn add_proposal(&mut self, proposal_name: String){
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
            let mut proposal_names: Vec<String> = Vec::new();
            proposal_names.push(String::from("Proposal # 1"));  
            let ballot = Ballot::new();
            assert_eq!(ballot.get_voter_count(),1);
        }

        #[ink::test]
        fn adding_proposals_works() {
            let mut ballot = Ballot::new();
            ballot.add_proposal(String::from("Proposal #1"));
            assert_eq!(ballot.get_proposal_count(),1);
        }
        
        #[ink::test]
        fn adding_voters_work() {
            let mut ballot = Ballot::new();
            let account_id = AccountId::from([0x0; 32]);
            assert_eq!(ballot.add_voter(account_id),true);
            assert_eq!(ballot.add_voter(account_id),false);
        }
    }
}
