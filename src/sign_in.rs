use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::str::FromStr;

/// Used to allow a private key execute txs.
pub async fn setup_signer(
    priv_key: String,
    provider: Provider<Http>,
) -> Option<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let chain_id: u64 = provider.get_chainid().await.unwrap().as_u64();

    let wallet: Result<Wallet<SigningKey>, WalletError> = priv_key.parse::<LocalWallet>();
    match wallet {
        Ok(x) => {
            let w = x.with_chain_id(chain_id).clone();
            return Some(SignerMiddleware::new(provider, w));
        }
        Err(_) => {
            println!("Failed to connect to wallet.");
            return None;
        }
    }
}

/// Create a message, returning a signature for verification purposes. 
/// 
/// Example:
/// 
/// The msg:
/// "DeGatchi#9032"
/// 
/// From the address:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
/// 
/// Creates the signature:
/// 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b
pub async fn create_msg(
    middleware: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    msg: &str,
) -> Option<(Signature, String)> {
    // sign message from your wallet and print out signature produced.
    match middleware.signer().sign_message(msg).await {
        Ok(sig) => {
            println!("Produced signature {} with {}.", sig, middleware.address());
            
            // verify the signature produced from your wallet.
            sig.verify(msg, middleware.address()).unwrap();
            println!("Verified signature produced by {:?}!", middleware.address());

            return Some((sig, msg.to_string()))
        },
        Err(_) => {
            return None;
        }
    }
}

/// Recovers pub key from Signature.
///
/// Example:
///
/// The signature:
/// r: 71031592387720320433450688414937280839659347503695324159450811751904079575653
/// s: 42005310148794597377403683159426268297577400373531751122007758979183741542804
/// v: 27
///
/// Gives the signer:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
pub fn verify_sig(signature: Signature, msg: &str) -> Option<H160> {
    let r: Result<H160, SignatureError> = signature.recover(msg);
    match r {
        Ok(signer) => return Some(signer),
        Err(_) => return None,
    }
}

/// Converts signature string into [r, s, v] and recovers pub key.
///
/// Example:
///
/// The signature:
/// 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b
///
/// Gives the signer:
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
pub fn verify_sig_str(signature: &str, msg: &str) -> Option<H160> {
    let sig: Signature = FromStr::from_str(signature).unwrap();
    println!("{:?}", sig);
    let r: Result<H160, SignatureError> = sig.recover(msg);
    match r {
        Ok(signer) => return Some(signer),
        Err(_) => return None,
    }
}
