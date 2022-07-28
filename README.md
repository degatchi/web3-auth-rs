# web3-login-rs
> Sign-In w/ Metamask.

## What
A simple library to help authenticate addresses via signing + eliptic curve recovering the signature for the public key.

## Why
Since [Web3Auth](https://github.com/Web3Auth/Web3Auth) is only in TS and my current projects are written in Rust (and i'm addicted to Rust) I wanted to be able to verify a wallet, in Rust, without sending a private key(s) over an application (e.g., copy and paste).

## Resources
- https://shobhitic.github.io/ethsign/
- https://medium.com/mycrypto/the-magic-of-digital-signatures-on-ethereum-98fe184dc9c7