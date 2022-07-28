# Web3 Auth w/ Rust
> Authenticate your wallet by signing and verifying messages.

## What
A simple library to help authenticate addresses via signing + ecrecover (eliptic curve recover) the signature for the public key.

## Why
Since [Web3Auth](https://github.com/Web3Auth/Web3Auth) is only in TS and my current projects are written in Rust. So, I needed a way to be able to verify a wallet, without sending a private key(s) over an application (e.g., copy and paste), in Rust.

## Resources
- [Create and verify msgs](https://shobhitic.github.io/ethsign/)
- [Ethereum Signature Handling](https://medium.com/mycrypto/the-magic-of-digital-signatures-on-ethereum-98fe184dc9c7)