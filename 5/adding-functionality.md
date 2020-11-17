Adding Functionality
===

So in this part we will add functionality to our Ballot so that people:
- Can cast vote on proposals
- Delegate their votes to other people
- Chairperson can assign voting rights to people


### Contract Functionality

#### **Constructor**:
To begin with lets update the constuctor of our contract. As you can see in the code sample, the constrcutor now accepts an `Option<Vector<String>>`. The constrcutor expects a vector of strings to be passed as input we can check if the vector containing strings is provided using:
```rust
if proposal_name.is_some() {
    names = proposal_name.unwrap()
    // do somethiing with names 

}
``` 
#### **Give Voting Right:**
In last part we created a funciton that allowed users to add themself as a voter in the ballot. But remember we initialized their voter struct with `voter.weight=0`. This is because by default when a voter is created he/she has no voting right. So lets create a function `give_voting_right` that will allow only the chair person to update the weight to 1 for a given voter. The function will look something like this:

```rust
    // assuming that the caller is the chair person
    pub fn give_voting_right(&mut self, voter_id: AccountId) {
        let voter_opt = self.voters.get_mut(&voter_id);
        if voter_opt.is_some() {
            let voter = voter.unwrap()
            // assuming that the voter has not already voted
            voter.weight = 1
        }
    }
```


#### **Vote:**
Next up, lets implement a function that will allow voters to cast their votes. This function will take as input a proposal index to which user wishes to vote for. If the `caller` is a valid voter and has not already casted the vote, we will update the proposal at index `i` with the weight of the voter and update `voter.voted` to true and set `voter.vote` to index of proposal.


#### **Get Winning Proposal:**
Now that the votes are casted, we will implement a function that will get the name of the proposal that won. In  a reall election, the winner is announced once the voting time has passed out, we will leave such implementation to you guys. For now we are going to allow any user to invoke this function and get the name of the propoasl that has won. Lets implement a function to return us the index of the proposal that has the maximum vote like this:
```rust

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
```
Notice that this function returns an `Option<usize>` not `usize` as its possible that there is no proposal in the ballot and users might try to invoke this function in which case we will return `None`. Now this function could be used to easily find the name of the winning proposal!


```rust 
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
```
You will see that in our delegation function we update the `sender.voted`, `sender.delegate` fields prior to checking if the person being delegated is a valid voter or not. The fact that the function will panic if the delegated person is not a valid voter will role back the changes made to sender voted and delegate fields.


## Your Turn!
This wraps up our tutorial but you have got some work to do!
- Update the `constructor` so that if vector of proposal names is provided, create new proposal objects and add to ballot.proposal
- Provide definition of `give_voting_right` function as instrcuted in the template
- Add `vote` functionality and update the ballot according the requirements mentioned in the template
- Update `get_winning_proposal_name` funcitonality so that it returns the winning proposal name 


<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.3-template.rs ':include :type=code embed-template')


#### ** Solution **

[embedded-code-final](./assets/5.3-solution.rs ':include :type=code embed-final')