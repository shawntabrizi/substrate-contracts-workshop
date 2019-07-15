Running a Substrate Node
===

After successfully installing `substrate`, you can start a local development chain by running:

```bash
substrate --dev
```

> **Note:** You should use the `substrate-node-template` from the [`substrate-package`](https://github.com/shawntabrizi/substrate-package/tree/contract) rather than the node installed on your computer. Change instances of `substrate` command with `./target/release/node-template`.

![An image of the terminal starting a Substrate node](./assets/start-substrate-node.png)

> **Note:** If you have run this command in the past, you probably want to purge your chain so that you run through this tutorial with a clean slate. You can do this easily with `substrate purge-chain --dev`.

You should start to see blocks being produced by your node in your terminal.

You can interact with your node using the Polkadot UI:

https://polkadot.js.org/apps/

> **Note:** You will need to use Google Chrome to have this site interact with your local node. The Polkadot UI is hosted on a secure server, and your local node is not, which may cause compatibility issues on Firefox or Linux based Chromium. The other option is to [clone and run the Polkadot UI locally](https://github.com/polkadot-js/apps).

If you go into the **Explorer** tab of the UI, you should also see blocks being produced!

![An image of the Substrate UI](./assets/start-substrate-ui.png)
