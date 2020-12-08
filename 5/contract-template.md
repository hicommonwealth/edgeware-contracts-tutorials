Contract Template
===

Let's start with making a new ink! project to build the ballot contract.

In your working directory, run:

```bash
cargo contract new ballot
```

Replace the content of `lib.rs` file with the template on the right.

The contract storage consists of an `AccountId` which we initialize to the callers id in the constructor.
There is a function `get_chair_person` implemented that returns the id of the the chair_person(owner) of the contract.

## Struct
You may have come across the [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) keyword in previous tutorials, but so far we have used structs to define the storage of contracts. In this contract, we use it to define the following custom types that are going to be used later use as part ballot storage:
- `Proposal`: This struct stores information about a proposal. Each proposal contains:
    -  `name`: A field to store the name of the propsal

    -  `vote_count`: A 32 bit unsigned integer for storing the number of votes the proposal has received. 

- `Voter`: For each voter in the system, we instantiate a voter struct with:
    - `weight`: An unsiged weight indicating the weightage of the voter. The weightage of a voter can vary based on election/network parameters.
    
    - `voted`: It is set to false by default, but once a voter has voted, it's set to true so that the same voter can not cast his vote again.
    
    - `delegate`: A voter can choose to delegate their vote to some one else. Since it's not necessary for voters to delegate, this field is created as an `Option`.
    
    - `vote`: Index of the proposal that the user has casted the vote to. This is created as an `Option` set to None by default.

Unlike our contract struct `Ballot` we don't use the macro `ink(storage)` for our custom defined structs as there can only be a single stroage struct for a contract. Also, our structs are not public as users don't need to interact with them directly.

## Ink_Prelude
`ink_pelude` crate provides data structures such as `HashMap`, `Vector` etc.. to operate on contract memory during contract execution. We will be importing these collections in next parts of this tutorial so before moving forward update contract's cargo.toml file with following dependency: `ink_prelude = { version = "3.0.0-rc2", default-features = false }`

## Compilaton and Warnings

You can build the contract using `cargo +nightly build` and run tests using `cargo +nightly test`. The contract will sucessfully compile and pass all tests, but the rust compiler will give you the following warnings:

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
