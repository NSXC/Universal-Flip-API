use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::convert::TryInto;
use rand::{RngCore};
use rand::rngs::OsRng;

fn generate_server_seed() -> String {
    let timestamp = chrono::Utc::now().timestamp_nanos();
    format!("{:?}", timestamp)
}

fn generate_client_seed() -> String {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; 32];
    rng.fill_bytes(&mut buffer);
    let client_seed: String = buffer//random buffers and bytes 
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    client_seed
}

fn coinflip(server_seed: &str, client_seed: &str) -> f64 {
    let mut hmac = Hmac::<Sha256>::new_varkey(server_seed.as_bytes()).expect("HMAC error");
    hmac.update(client_seed.as_bytes());
    let hash = hmac.finalize().into_bytes();
    let numerator: u128 = u128::from_be_bytes(hash[..16].try_into().unwrap());//Gettings Randomness from seeds
    let denominator: u128 = u128::MAX;
    if numerator > denominator {
        panic!("Numerator exceeds maximum value of u128.");
    }
    let result = numerator as f64 / denominator as f64;
    let rounded_result = (result * 2.0).round() / 2.0; //prevent #5 errors (users will never tie) [my little hack]
    rounded_result
}

fn main() {
    let server_seed = generate_server_seed();
    println!("Server seed: {}", server_seed);
    let client_seed = generate_client_seed();
    println!("Client seed: {}", client_seed);
    let result = coinflip(&server_seed, &client_seed);
    println!("{}", result);
}
