use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::convert::TryInto;
use rand::{RngCore};
use rand::rngs::OsRng;
use rand::Rng;

fn generate_server_seed() -> String {
    let timestamp = chrono::Utc::now().timestamp_nanos();
    format!("{:?}", timestamp)
}

fn generate_client_seed() -> String {
    let mut rng = OsRng;
    let mut buffer = vec![0u8; 32];
    rng.fill_bytes(&mut buffer);
    let client_seed: String = buffer
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    client_seed
}

fn coinflip(server_seed: &str, client_seed: &str) -> f64 {
    let mut hmac = Hmac::<Sha256>::new_varkey(server_seed.as_bytes()).expect("HMAC error");
    hmac.update(client_seed.as_bytes());
    let hash = hmac.finalize().into_bytes();
    let numerator: u128 = u128::from_be_bytes(hash[..16].try_into().unwrap());
    let denominator: u128 = u128::MAX;
    if numerator > denominator {
        panic!("Numerator exceeds maximum value of u128.");
    }
    let mut result = numerator as f64 / denominator as f64;
    if result == 0.5{
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        result = if random_number < 0.5 { 0.6 } else { 0.4 };
    }
    result
}

fn main() {
    let server_seed = generate_server_seed();
    println!("Server seed: {}", server_seed);
    let client_seed = generate_client_seed();
    println!("Client seed: {}", client_seed);
    let result = coinflip(&server_seed, &client_seed);
    if result > 0.5{
       //handle user1
    }else if result < 0.5{
        //handle user2
    }else{
        println!("Error In Results")
    }
    println!("{}", result);
}
