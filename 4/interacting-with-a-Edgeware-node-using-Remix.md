### Introduction 

This guide walks through the process of creating and deploying a Solidity-based smart contract to a Edgeware dev node using the [Remix IDE](https://remix.ethereum.org/). Remix is one of the commonly used development environments for smart contracts on Ethereum. Given Edgeware’s Ethereum compatibility features, Remix can be used directly with a Edgeware node.

This guide assumes that you have a running local Edgeware node running in `--dev` mode, and that you have a MetaMask installation configured to use this local node. [You can find instructions for running a local Edgeware node](4/setting-up-a-local-node.md) and [to configure MetaMask for Edgeware](4/interacting-with-a-Edgeware-node-using-metamask.md).

### Interacting With Edgeware Using Remix

[Open Remix](https://remix.ethereum.org/), 


```Solidity
pragma solidity ^0.6.0;

import 'https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v3.1.0/contracts/token/ERC20/ERC20.sol';

// This ERC-20 contract mints the specified amount of tokens to the contract creator.
contract MyFirstToken is ERC20 {
  constructor(uint256 initialSupply) ERC20("MyFirstToken", "HEDGE") public {
    _mint(msg.sender, initialSupply);
  }
}
```

### We want to hear from you

This is obviously a simple example, but it provides context for how you can start working with Edgeware and how you can try out it's Ethereum compability features. We are interested in hearing abou your experience and following the steps in this guide or your experience trying other Etheruem-based tools with Edgeware. [Feel free to join us in the channels (Discord, Element, Telegram)](https://linktr.ee/edg_developers). We would love to hear your feedback on Edgeware EVM and answer any questions that you may have.