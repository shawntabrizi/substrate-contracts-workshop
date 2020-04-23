Troubleshooting
===

Here are solutions to some of the common problem you may come across:

## Unexpected Epoch Change

There is a known issue with the Substrate block production (BABE) on a running chain. If you stop your node for too long (closing the terminal, putting your computer to sleep, etc...), you will get the following error:

```bash
ClientImport("Unexpected epoch change")
```

To solve this you will need to reset your node with: `substrate purge-chain --dev`. At that point, you will need to re-deploy any contracts and re-do any steps that you may have done before on your node. As long as you keep your node running, you should face no issues.

## Old Contracts in Local Storage

The Polkadot UI uses its own local storage to track the contracts that you have deployed. This means that if you deploy a contract using the UI, and then purge your Substrate node, you will still see the old contracts in the UI even though they do not exist on-chain!

You can simply remove any old artifacts from the UI or reset your local storage. So remember, when you `purge-chain` you will need to re-deploy any contracts and re-do any steps that you may have done before on your node.

## Transaction vs RPC

When interacting with contracts using the Polkadot UI, you have the option to submit your calls as a transaction or via the RPC:

![An image of submitting with transaction](./assets/send-as-rpc.png)
![An image of submitting with RPC](./assets/send-as-transaction.png)

When you send as a transaction, it should be exactly as you expect. A transaction is submitted to contract, a fee is deducted from your account, and the state of your blockchain can change. In these situations, no value is returned from your contract calls, only a "Success" or "Failed" extrinsic message along with any events you emit.

However, there may be some calls that you want to "test", rather than actually submit a transaction. Or you may want to peek at the value that _would_ be returned if you called the contract function. For these scenarios, you can submit an RPC call, which will run all of your contract logic, but not actually submit a transaction or update the state of your chain. However, you will still need to specify the right amount of gas to cover your "virtual fee", but don't worry, nothing will be charged when making a call this way. :)

## Contract State Rent

The Substrate contract pallet has a state rent system that forces contracts to stay funded if they want to stay on the blockchain. This means that the more you use a contract, the more fees are taken from it, and at some point, the contract will run out of fees and turn into a non-functioning tombstone. We try to avoid this by giving the contract a large endowment when we initially deploy it. However, if your contract does become a tombstone, for the purposes of this tutorial, the best solution is to just redeploy your contract to the chain.

The best way to prevent this in general is to make sure your contract stays well funded. In real world scenarios, there is a process that you can go through to recover a tombstone contract and get it functioning again, however this is beyond the scope of this tutorial.

## Other Issues

If you run into any other issues during this tutorial, please [report an issue](https://github.com/substrate-developer-hub/substrate-contracts-workshop/issues)!
