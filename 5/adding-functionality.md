Adding Functionality
===

So in this part we will add functionality to our Ballot so that people:
- Can cast vote on proposals
- Delegate their votes to other people
- Chairperson can assign voting rights to people

### Contract Functionality

#### Constructor
To begin with lets update the constuctor of our contract. As you can see in the code sample, the constrcutor now accepts an `Option<Vector<String>>`. The constrcutor expects a vector of strings to be passed as input we can check if the vector containing strings is provided using:
```rust
if proposal_name.is_some() {
    names = proposal_name.unwrap()
    // do somethiing with names 

}
``` 
#### Give Voting Right
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

#### Vote
Next up, lets implement a function that will allow voters to cast their votes. 


## Your Turn!
Adding Functionality


<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.3-solution.rs ':include :type=code embed-final')

<!-- tabs:end -->