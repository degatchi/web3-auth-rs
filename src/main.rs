use web3_auth_rs::sign_in::confirm_pub_key;

fn main() {
    let sig: &str = "0x9d0a74e1c6cb58cc8f5695fb0c1b7f2fc9795c47c45e35f05919555b2f90fa655cde29bba787dcb5e544c3b0604ba4b82a71178dde9e5ac04f3f1db151d0a1941b";
    let msg: &str = "DeGatchi#9032";
    match confirm_pub_key(sig, msg) {
        Some(pub_key) => println!("pub key: {:?}", pub_key),
        None => {},
    }
}
