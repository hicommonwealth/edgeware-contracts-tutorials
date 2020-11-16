Using Collections and Traits
===

In this part we will be using the structures built previously and write public functions to add and fetch data from our contract.

## Collections
For this contract we are going to store our voters in a hash map having AccountId as key and Voter instance as value. The hashmap can be imported from `ink_storage` crate like this: 

```rust
use ink_storage::collections::HashMap;
```

The proposals will be stored in a Vector collection which we will need to import from `ink_prelude` crate( it is a collection of data structures that operate on contract memory during contract execution ) like this:

```rust
    use ink_prelude::vec::Vec;
```
Vectors can be instantiated by simply the same way as a hashmap and new objects can be easily added or referenced like this:
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
Remmeber that the `vector.get` returns an Option just like a hash map.

## Traits
A trait tells the Rust compiler about functionality a particular type has and can share with other types. You can read more about them over [here](https://doc.rust-lang.org/book/ch10-02-traits.html). Before you can use the custom built structures inside the `Ballot` storage, we need to implement certain traits for our `Voter` and `Proposal` strcuts. These traits include:

- `Debug`: Allows debug formatting in format strings
- `Clone` : This trait allows you to create a deep copy of object
- `Copy` : The copy traits allows you to copy a value of a field
- `PackedLayout`: Types that can be stored to and loaded from a single contract storage cell
- `SpreadLayout`: Types that can be stored to and loaded from the contract storage.

You can learn more about these traits over [here](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) and [here](https://paritytech.github.io/ink/ink_storage/traits/index.html). These traits are implemented using `derive` atrribute like this:

```rust
    #[derive(Clone, Copy, Debug, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, scale_info::TypeInfo)]
    struct XYZ {
        ...
        ...
    }
```

## Your Turn!

You need to:
- Create a proposal vector and voters hash map in Ballot struct
- Update the constructor so that it initializes a vector of proposals and hashmap of voters. Add chair person to the hashmap of voters
- Create getters for both storage items
- Create a `add_voter` function to create a voter by the given AccountId and insert it in hashmap of voters if not already exists.
- Create a `add_proposal` function that creates and insert the proposal inside the vector of proposals

Don't forget to run `cargo +nightly test`

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/5.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/5.2-solution.rs ':include :type=code embed-final')

<!-- tabs:end -->