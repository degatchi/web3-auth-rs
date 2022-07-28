use web3_login_rs::sign_in::confirm_pub_key;

fn main() {
    match confirm_pub_key("DeGatchi#9032") {
        Some(pub_key) => println!("pub key: {:?}", pub_key),
        None => {},
    }
}
