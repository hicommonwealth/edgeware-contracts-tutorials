Adding Functionality
===

In this part we will add functionality to our Ballot so that:
- People can vote on proposals
- People can delegate their votes
- The chairperson can assign voting rights


### Contract Functionality

#### **Constructor**:
Let's first update the constuctor of our contract. As you can see in the code sample on the right, the constrcutor now accepts a `Option<Vector<String>>` parameter. The constrcutor expects a vector of strings as input. We need to update the constructor so that the provided proposal names are used to create the `Proposal` objects and added to our ballot storage. To check if the vector containing strings is provided:

```rust

    if proposal_name.is_some() {
        names = proposal_name.unwrap()
        // do somethiing with names 

    }
```

#### **Give Voting Right:**
In the previous part we created a function that allowed users to add themselves as a voter. We initialized their voter struct with `voter.weight=0` because when a voter is created, he/she has no voting right by default. So, let's create a function `give_voting_right` that will only allow the chairperson to update the weight to 1 for any given voter. The function will look something like:

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
Now, let's implement a function that will allow users to cast their votes. This function will take a proposal index as input. If the `caller` is a valid voter and has not already casted his/her vote, update the proposal at index `i` with the weight of the voter, update `voter.voted` to true and set `voter.vote` to index `i`.


#### **Get Winning Proposal:**
Now that the votes are casted, we will implement a function that will get the name of the winning proposal. In  a recall election, the winner is announced once the voting time has passed out. We will leave such implementation to you. For now, we will allow any user to invoke this function and get the name of the winning proposal. Let's implement a function to return the index of the proposal with the maximum votes:
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
Notice that this function returns `Option<usize>`, not `usize`, since it's possible that there are no proposals in the ballot. This function can be used to find the name of winning proposal.

#### **Delegation:**
In our voter struct, there is a `delegate` field defined as `Option<AccountId>` to allow voters to delegate their vote to someone else. This can be achieved using the following function: 
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
You will see that in the delegation function above, we update the `sender.voted` and `sender.delegate` fields prior to checking if the person being delegated is a valid voter. The function will panic if the delegated person is not a valid voter and will roll back the changes made to the `sender.voted` and `sender.delegate` fields.


## Your Turn!
This wraps up the tutorial. Practice what you learnt with the following exercises: 
- Update the `constructor` function so that if a vector of proposal names is provided, a new proposal object is created and added to `ballot.proposal`.
- Define the `give_voting_right` function as instructed.
- Add `vote` functionality and update the ballot according to the template requirements.
- Update `get_winning_proposal_name` funcitonality to return the name of the winning proposal.


<!-- tabs:start -->

#### * Template *

[embedded-code](./assets/5.3-template.rs ':include :type=code embed-template')

#### * Solution *

[embedded-code-final](./assets/5.3-solution.rs ':include :type=code embed-final')

<!-- tabs:end -->