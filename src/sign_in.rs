use ethers::{prelude::{*, k256::ecdsa::SigningKey}, utils::keccak256};

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
    

pub fn sign_in(msg: &str) -> Option<H160> {

    // let data = msg.as_bytes();
    // let hash = keccak256(data);
    
    // 0xe5a12cfa871dfa50c8085ec25356a984d241b13ac0db1f5b8185ffdfb8fd8c63 7f9c5ff25287b33bc9992ba04ed84d1b2c180e724d487eda85f08e18a06c9af6 1c
    // 0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa65 5cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a194 1b
    let r = "0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa65".parse::<Bytes>().unwrap();
    let r2 = U256::from_big_endian(&r);

    let s = "0x5cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a194".parse::<Bytes>().unwrap();
    let s2 = U256::from_little_endian(&s);

    let sig = Signature {
        r: r2,
        s: s2,
        v: 28,
    };

    let r = sig.recover(msg);
    match r {
        Ok(signer) => return Some(signer),
        Err(_) => return None,
    }
}
