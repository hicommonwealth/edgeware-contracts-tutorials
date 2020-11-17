#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]    
mod ballot {
    // use Hash
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
        voters: HashMap<AccountId, Voter>,
        proposals: Vec<Proposal>    
    }

    impl Ballot {
        #[ink(constructor)]
        pub fn new(proposal_names: Option<Vec<String>> ) -> Self {

            // get chair person address
            let chair_person =  Self::env().caller();

            // create empty propsal and voters
            let mut proposals: Vec<Proposal> = Vec::new();
            let mut voters = HashMap::new();

            // initialize chair person's vote
            voters.insert(chair_person, Voter{
                weight:1,
                voted:false,
                delegate: None,
                vote: None,
            });


             // ACTION : Check if proposal names are provided.
             //         * If yes then create and push proposal objects to proposals vector
                // if proposals are provided
                if proposal_names.is_some() {
                    // store the provided propsal names
                    let names = proposal_names.unwrap();
                    for name in &names {
                        proposals.push(
                            Proposal{
                            name: String::from(name),
                            vote_count: 0,
                        });
                    }
                }

            Self {
                chair_person,
                voters,
                proposals,
            }
        }

        /// default constrcutor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }


        #[ink(message)]
        pub fn get_chairperson(&self) -> AccountId {
            self.chair_person
        }



        pub fn get_voter(&self, voter_id: AccountId) -> Option<&Voter>{
            self.voters.get(&voter_id)
        }

        pub fn get_voter_count(&self) -> usize{
            self.voters.len() as usize
        }

                /// the function adds the provided voter id into possible
        /// list of voters. By default the voter has no voting right,
        /// the contract owner must approve the voter before he can cast a vote
        #[ink(message)]
        pub fn add_voter(&mut self, voter_id: AccountId) -> bool{

            let voter_opt = self.voters.get(&voter_id);
            // the voter does not exists
            if voter_opt.is_some() {
                return false
            }

            self.voters.insert(voter_id, Voter{
                weight:0,
                voted:false,
                delegate: None,
                vote: None,
            });
            return true
        }



        /// given an index returns the name of the proposal at that index
        pub fn get_proposal_name_at_index(&self, index:usize) -> &String {
            let proposal = self.proposals.get(index).unwrap();
            return &proposal.name
        }

        /// returns the number of proposals in ballet
        pub fn get_proposal_count(&self) -> usize {
            return self.proposals.len()
        }

        /// adds the given proposal name in ballet
        /// to do: check unqiueness of proposal,
        pub fn add_proposal(&mut self, proposal_name: String){
            self.proposals.push(
                Proposal{
                    name:String::from(proposal_name),
                    vote_count: 0,
            });
        }
        
        /// Give `voter` the right to vote on this ballot.
        /// Should only be called by `chairperson`.
        #[ink(message)]
        pub fn give_voting_right(&mut self, voter_id: AccountId) {
            let caller = self.env().caller();
            let voter_opt = self.voters.get_mut(&voter_id);

            // ACTION: check if the caller is the chair_person
            //         * check if the voter_id exists in ballot
            //         * check if voter has not already voted
            //         * if everything alright update voters weight to 1
    

            // only chair person can give right to vote
            assert_eq!(caller,self.chair_person, "only chair person can give right to vote");

            // the voter does not exists
            assert_eq!(voter_opt.is_some(),true, "provided voterId does not exist");

            let voter = voter_opt.unwrap();
            // the voter should not have already voted
            assert_eq!(voter.voted,false, "the voter has already voted");

            voter.weight = 1;
        }


        /// Give your vote (including votes delegated to you)
        /// to proposal `proposals[proposal]`.
        #[ink(message)]
        pub fn vote(&mut self, proposal_index: i32) {
            let sender_id = self.env().caller();
            let sender_opt =  self.voters.get_mut(&sender_id);

            //  ACTION: check if the person calling the function
            //          is a voter
            //        * check if the person has not already voted
            //        * check if the person has right to vote
            // 

            assert_eq!(sender_opt.is_some(),true, "Sender is not a voter!");

            let sender = sender_opt.unwrap();
            assert_eq!(sender.voted,false, "You have already voted");

            assert_eq!(sender.weight,1, "You have no right to vote");


            // get the proposal
            let proposal_opt = self.proposals.get_mut(proposal_index as usize);

            //  ACTION: check if the proposal exists
            //        * update voters.voted to true
            //        * update voters.vote to index of proposal to which he voted
            //        * Add weight of the voter to  proposals.vote_count 

            assert_eq!(proposal_opt.is_some(),true, "Proposal index out of bound");

            let proposal = proposal_opt.unwrap();

            sender.voted = true;
            sender.vote = Some(proposal_index);

            proposal.vote_count += sender.weight;

        }


            /// @dev Computes the winning proposal taking all
            /// previous votes into account.
            fn winning_proposal(&self) -> Option<usize> {
                let mut winning_vote_vount:u32 = 0;
                let mut winning_index: Option<usize> = None;
                let mut index: usize = 0;

                for val in self.proposals.iter() {
                    if val.vote_count > winning_vote_vount {
                        winning_vote_vount = val.vote_count;
                        winning_index = Some(index);
                    }
                    index += 1

                }
                return winning_index
            }


        // Calls winning_proposal() function to get the index
        // of the winner contained in the proposals array and then
        // returns the name of the winner
        pub fn get_winning_proposal_name(&self) -> &String {
            
            //  ACTION: use winning_proposal to get the index of winning proposal
            //        * check if any proposal has won
            //        * return winnning proposal name if exists
            let winner_index: Option<usize> = self.winning_proposal();
            assert_eq!(winner_index.is_some(),true, "No Proposal!");
            let index = winner_index.unwrap();
            let proposal = self.proposals.get(index).unwrap();
            return &proposal.name

        }


         /// Delegate your vote to the voter `to`.
        /// If the `to` has already voted, you vote is casted to
        /// the same candidate as `to`
        #[ink(message)]
        pub fn delegate(&mut self, to: AccountId)  {

            // account id of the person who invoked the function
            let sender_id = self.env().caller();
            let sender_weight;
            // self delegation is not allowd
            assert_ne!(to,sender_id, "Self-delegation is disallowed.");
    
            {
                let sender_opt =  self.voters.get_mut(&sender_id);
                // the voter invoking the function should exist in our ballot
                assert_eq!(sender_opt.is_some(),true, "Caller is not a valid voter");
                let sender = sender_opt.unwrap();
    
                // the voter must not have already casted their vote
                assert_eq!(sender.voted,false, "You have already voted");
    
                sender.voted = true;
                sender.delegate = Some(to);
                sender_weight = sender.weight;
            }
    
            {
                let delegate_opt = self.voters.get_mut(&to);
                // the person to whom the vote is being delegated must be a valid voter
                assert_eq!(delegate_opt.is_some(),true, "The delegated address is not valid");
    
                let delegate = delegate_opt.unwrap();
    
                // the voter should not have already voted
                if delegate.voted {
                    // If the delegate already voted,
                    // directly add to the number of votes
                    let voted_to = delegate.vote.unwrap() as usize;
                    self.proposals[voted_to].vote_count += sender_weight;
                } else {
                    // If the delegate did not vote yet,
                    // add to her weight.
                    delegate.weight += sender_weight;
                }
            }
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
            let ballot = Ballot::new(Some(proposal_names));
            assert_eq!(ballot.get_proposal_count(),1);
        }

        #[ink::test]
        fn default_works() {
            let ballot = Ballot::default();
            assert_eq!(ballot.get_proposal_count(), 0);
        }

        #[ink::test]
        fn adding_proposals_works() {
            let mut ballot = Ballot::default();
            ballot.add_proposal(String::from("Proposal #1"));
            assert_eq!(ballot.get_proposal_count(),1);
        }

        #[ink::test]
        fn adding_voters_work() {
            let mut ballot = Ballot::default();
            let account_id = AccountId::from([0x0; 32]);
            assert_eq!(ballot.add_voter(account_id),true);
            assert_eq!(ballot.add_voter(account_id),false);
        }

        #[ink::test]
        fn give_voting_rights_work() {
            let mut ballot = Ballot::default();
            let account_id = AccountId::from([0x0; 32]);

            ballot.add_voter(account_id);     
            ballot.give_voting_right(account_id);
            let voter = ballot.get_voter(account_id).unwrap();
            assert_eq!(voter.weight,1);
        }

        #[ink::test]
        fn voting_works() {
            let mut ballot = Ballot::default();
            ballot.add_proposal(String::from("Proposal #1"));
            ballot.vote(0);
            let voter = ballot.get_voter(ballot.get_chairperson()).unwrap();
            assert_eq!(voter.voted,true);
        }

        #[ink::test]
        fn delegation_works() {
            let mut ballot = Ballot::default();
            let to_id = AccountId::from([0x0; 32]);

            ballot.add_voter(to_id);     
            ballot.delegate(to_id);

            let voter = ballot.get_voter(ballot.get_chairperson()).unwrap();
            assert_eq!(voter.delegate.unwrap(),to_id);
        } 

        #[ink::test]
        fn get_winning_proposal_name_working() {
            let mut ballot = Ballot::default();
            ballot.add_proposal(String::from("Proposal #1"));
            ballot.add_proposal(String::from("Proposal #2"));
            ballot.vote(0);
            let proposal_name = ballot.get_winning_proposal_name();
            assert_eq!(proposal_name, "Proposal #1");
        }

    }
}
