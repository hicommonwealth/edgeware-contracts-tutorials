### Setting up a Edgeware Node for Ethereum/EVM development

This guide gives you steps to create a node for testing the Ethereum compatibility functionality of Edgeware EVM

> **Note** This is fast track way to run your node for great Developer Experience, [you can always opt-out for compile it from source](https://github.com/hicommonwealth/edgeware-node/tree/edgeware-frontier) to be sure. We recommend to use your own compiled binaries for production mainnet.

> **Note** If you don't have [Docker installed, you can quickly install it from here](https://docs.docker.com/get-docker/)

You will clone our repo with various docker-compose files and run it right away:

```shell
git clone https://github.com/yangwao/substrate_playground; cd substrate_playground;
docker-compose -f edgeware_frontier.yml up
```
> **Note** If you want to flush data, delete docker container `docker-compose -f edgeware_frontier.yml rm` 

You will see something like this

![Running-Edgeware-EVM-node](./assets/node-setup-run.png)

> **Note** If you want to reset or purge chain locally, remove container with `docker-compose -f edgeware_frontier.yml rm`

Afterwards you can head to [Polkadot Apps and connect to 127.0.0.1:9944](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) and you should see block producing in explorer.

![Edgeware-EVM-producing-blocks](./assets/frontier-explorer.png)

Now you can continue to connect your [Metamask](4/interacting-with-a-Edgeware-node-using-metamask.md), Remix or Web3 to have great experience.

### Reach us for more engagement

Glad you've made it through! 🥰 We are eager to guide your more on your exploration through Edgeware Ethereum compability feature. We are **keen to hear your experience and suggestion you may for us.**. You can feel free to [chat with us in the Edgeware's channels like Discord, Element and Telegram](https://linktr.ee/edg_developers), we can help you out with issues you may have or project you may want to be funded through our [Treasury program](https://docs.edgewa.re/edgeware-runtime/treasury). Don't hesitate to share your feedback on our channels, there is always space to improve! 🙌