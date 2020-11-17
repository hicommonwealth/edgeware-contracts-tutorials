Using Collections and Traits
===

In this part,  we will be using the structs built previously and write public functions to add and fetch data from our contract.

## Collections
For this contract, we are going to store our voters in a HashMap with `AccountId` as a key and `Voter` instance as value. The HashMap can be imported from the `ink_storage` crate by: 

```rust
use ink_storage::collections::HashMap;
```

The proposals will be stored in a `Vec` collection that can be imported from the `ink_prelude` crate. `ink_prelude` is a collection of data structures that operate on contract memory during contract execution.The vector can be imported by: 

```rust
    use ink_prelude::vec::Vec;
```
Vectors can be instantiated in the same way as a HashMap.  New objects can be added or referenced to from a vector using:
```rust
    let proposals: Vec<proposals> = Vec::new();
    // adding a new proposal
    proposals.push(
        Proposal{
            name:String::from("Proposal # 1"),
            vote_count: 0,
        });
    // returns the proposal at index 0 if exists else returns None
    let proposal = self.proposals.get(0).unwrap();
    
```
Rememeber that the `vector.get` returns an `Option` not the actual object!

## Traits
A trait tells the Rust compiler about the functionality a particular type has, and can be shared with other types. You can read more about them [here](https://doc.rust-lang.org/book/ch10-02-traits.html). Before using the custom built structures inside the `Ballot` storage, certain traits are required to be implemented for `Voter` and `Proposal` strcuts. These traits include:

- `Debug`: Allows debug formatting in format strings
- `Clone` : This trait allows you to create a deep copy of object
- `Copy` : The copy traits allows you to copy a value of a field
- `PackedLayout`: Types that can be stored to and loaded from a single contract storage cell
- `SpreadLayout`: Types that can be stored to and loaded from the contract storage.

You can learn more about these traits over [here](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) and [here](https://paritytech.github.io/ink/ink_storage/traits/index.html). These traits are implemented using  the `derive` atrribute:

```rust
    #[derive(Clone, Copy, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, scale_info::TypeInfo)]
    struct XYZ {
        ...
        ...
    }
```

## Your Turn!

You need to:
- Create a proposal Vec and voters HashMap in Ballot struct.
- Update the constructor, so that it initializes a Vec of proposals and a HashMap of voters. Also update the voters HashMap to include the chair person as a voter.
- Create getters for both storage items.
- Write `add_voter` function to create a voter by the given `AccountId` and insert it in the HasMmap of voters.
- Write `add_proposal` function that creates a `Proposal` object and inserts it tot he vector of porposals.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.2-solution.rs ':include :type=code embed-final')

<!-- tabs:end -->