### Setting up a Edgeware Node for Ethereum/EVM development

This guide walks you through setting up an Edgeware node with Ethereum/EVM compatibility.

> **Note** This is a fast-track way to run a node. [You can always compile from source](https://github.com/hicommonwealth/edgeware-node/tree/edgeware-frontier) as well. We recommend using your own compiled binaries for production mainnet.

> **Note** If you don't have [Docker installed, you can quickly install it from here](https://docs.docker.com/get-docker/)

You can clone our repo with docker-compose to get started right away:

```shell
git clone https://github.com/yangwao/substrate_playground; cd substrate_playground;
docker-compose -f edgeware_frontier.yml up
```
> **Note** If you want to reset or purge the local chain, delete the docker container by running `docker-compose -f edgeware_frontier.yml rm`

You will see something like this:

![Running-Edgeware-EVM-node](./assets/node-setup-run.png)

Afterwards you can head to [Polkadot Apps](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) and connect to 127.0.0.1:9944, and you should see blocks being produced.

![Edgeware-EVM-producing-blocks](./assets/frontier-explorer.png)

Now you can continue to connect [Metamask](4/interacting-with-a-Edgeware-node-using-metamask.md), Remix, and Web3.js to have great experience.

### We want to hear from you

This is obviously a simple example, but it provides context for how you can start working with Edgeware and how you can try out its Ethereum compability features. [Feel free to join us in the chat channels (Discord, Element, Telegram)](https://linktr.ee/edg_developers). We would love to hear your feedback on Edgeware EVM and answer any questions that you may have.