use std::str::FromStr;
use ethers::{prelude::{*, k256::ecdsa::SigningKey}};

/// Used to allow a private key execute txs.
pub async fn setup_signer(
    priv_key: String,
    provider: Provider<Http>
) -> Option<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let chain_id: u64 = provider.get_chainid().await.unwrap().as_u64();

    let wallet: Result<Wallet<SigningKey>, WalletError> = priv_key.parse::<LocalWallet>();
    match wallet {
        Ok(x) => {
            let w = x.with_chain_id(chain_id).clone();
            return Some(SignerMiddleware::new(provider, w));
        },
        Err(_) => {
            println!("Failed to connect to wallet.");
            return None;
        }
    }
}
   
/// Example:
/// 
/// The signature: 
/// 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b
/// 
/// Gives the signer: 
/// 0xdcd49c36e69bf85fa9c5a25dea9455602c0b289e
pub fn confirm_pub_key(signature: &str, msg: &str) -> Option<H160> {
    let sig: Signature = FromStr::from_str(signature).unwrap();
    println!("{:?}", signature);
    let r: Result<H160, SignatureError> = sig.recover(msg);
    match r {
        Ok(signer) => return Some(signer),
        Err(_) => return None,
    }
}
