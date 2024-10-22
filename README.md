# Two-sided-Marketplace-for-Services

Marketplace for buying and selling services.
Like   a 1:1 Rust & Anchor lesson (1 hour videocall), or any other service you'd like to offer or buy.

## Building
Building this Anchor program requires Anchor version 30.1. Update your Anchor version through the CLI: 

```sh
avm install 0.30.1
``` 

Already installed 0.30.1? Check using `avm list`. If version 0.30.1 is installed, but not currently running start using by: 

```sh
avm use 0.30.1
``` 

Any Solana version above 1.17.3 is supported by Anchor, but 1.18.17 is recommended. 

```sh
solana-install init 1.18.17
```


## Testing
By default, this repo is using EUROC (https://www.circle.com/en/eurc), but any SPL token can work. Using the default settings, deploy the program to Devnet for testing. Replace the mint with a token of your choosing to run the program on your local machine. 

```sh
anchor deploy
```

Replace the program address with the address of your deployed program (replace the addresses `lib.rs` and `Anchor.toml`).

```sh
anchor test
```
