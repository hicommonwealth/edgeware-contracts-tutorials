Contract Template
===

Lets start with making a new ink! project to build the ballot contract.

In your working directory, run:

```bash
cargo contract new ballot
```

Replace the content of `lib.rs` file with the template provided on this page.

The contract storage consists of an `AccountId` which we initialize to the callers id in the constructor.
There is a function `get_chair_person` implemented that returns the id of the the chair_person(owner) of the contract.

## Struct
You have come across the [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) keyword in previous tutorials but so far we have used to define the storage of the contract. In this contract we use it to define following custom types which we are going to later use as part of our storage:
- `Proposal`: This struct stores information about a proposal. Each proposal has:
    -  `name`: A field to store the name of the propsal

    -  `vote_count`: A 32 bit unsigned integer for storing the number of votes the proposal has received. 

- `Voter`: For each voter in the system we instantiate a voter struct. Every voter has:
    - `weight`: An unsiged weight indicating the weightage this voter has. The weightage for one voter can 
    vary from other one based on how the network operates. 
    
    - `voted`: Its false by default, but once a voter has voted, its set to true so that the same voter can not case the vote again
    
    - `delegate`: If a voter wants, they can delegate their vote to some other voter. This is Option because its not necessary for every voter to have a delegate.
    
    - `vote`: index of the proposal to which the user has casted the vote. Again its an option because when the voter is created in the system he has not voted to any proposal so its set to None by default.  

Unlke our contract struct `Ballot` we dont' use the macro `ink(storage)` for our custom defined structs as there could only be single stroage struct for a contract. Also our structs are not public as users don't need to interact or know about these structs.

## Compilaton and Warnings

You can build the build the contract using `cargo +nightly build` and run tests using `cargo +nightly test`. The contract will sucessfully compile and it will pass all tests, but rust compiler will give you the following warnings

```bash
warning: struct is never constructed: `Proposal`
  --> lib.rs:10:12
   |
10 |     struct Proposal {
   |            ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Voter`
  --> lib.rs:16:16
   |
16 |     pub struct Voter {
   |                ^^^^^

warning: 2 warnings emitted
```

This is because the structs we have defined are never used. We will get to that in next part!

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.1-template.rs ':include :type=code embed-template')

<!-- tabs:end -->