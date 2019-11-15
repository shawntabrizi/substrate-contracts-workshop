Calling Your Contract
===

Now that your contract has been fully deployed, we can start to interact with it! Flipper only has two functions, so we will show you what it's like to play with both of them.

## get()

If you take a look back at our contract's `on_deploy()` function, we set the initial value of the Flipper contract to `false`. Let's check that this is the case.

In the **Contracts** section, press the "execute" button:

![An image of the Contracts call page](./assets/flipper-call-page.png)

Set the _message to send_ to `get(): bool`. Set the _maximum gas allowed_ to `500,000`.

When you press "Call" you will see it returns the value `false`:

![An image of Flipper RPC call with false](./assets/flipper-false.png)

> NOTE: You might be wondering: "Why did we need to specify gas when reading a value from a contract?"
> 
> If you notice right above the "Call" button is a toggle which allows you to "send call as transaction" or "send as RPC call". For a read-only request like this, we can simply use an RPC call which will _simulate_ a transaction, but not actually store anything on-chain. Thus, you will still need to specify the right amount of gas to cover your "virtual fee", but don't worry, nothing will be charged when making a call this way. :)

## flip()

So let's make the value turn `true` now!

The alternative _message to send_ we can make with the UI is `flip()`. Again, set the _maximum gas allowed_ to `500,000`.

You will notice that this call actually sends a transaction. If the transaction was successful, we should then be able to go back to the `get()` function and see our updated storage:

![An image of Flipper RPC call with true](./assets/flipper-true.png)

Woohoo! You deployed your first smart contract!

## Moving Forward

We will not go over these setup and deployment steps again, but we will use them throughout the tutorial. You can always come back to this chapter if you need to remember how to do a certain process.

The rest of the tutorial will have **template code** which you will use to walk through the different steps of contract development. Each template comes with a fully designed suite of tests that should pass if you programmed your contract correctly. Before you move on from a section, make sure that you run:

```bash
cargo +nightly test
```

and that the tests all execute successfully, without any warnings.

You need not deploy your contract between each section, but if we ask you to deploy your contract, you will need to follow the same steps you have done with the Flipper contract.
