use web3_login_rs::sign_in::sign_in;

fn main() {
    match sign_in("DeGatchi#9032") {
        Some(pub_key) => println!("pub key: {:?}", pub_key),
        None => {},
    }
}
