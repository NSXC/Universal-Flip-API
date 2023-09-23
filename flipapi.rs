use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::convert::TryInto;
use rand::{RngCore};
use rand::rngs::OsRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use serde_json::json;
use chrono::Utc;
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
        .map(|byte: &u8| format!("{:02x}", byte))
        .collect();

    client_seed
}

fn coinflip(server_seed: &str, client_seed: &str) -> f64 {
    let mut hmac: Hmac<Sha256> = Hmac::<Sha256>::new_varkey(server_seed.as_bytes()).expect("HMAC error");
    hmac.update(client_seed.as_bytes());
    let hash: hmac::digest::generic_array::GenericArray<u8, hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UInt<hmac::digest::generic_array::typenum::UTerm, hmac::digest::consts::B1>, hmac::digest::consts::B0>, hmac::digest::consts::B0>, hmac::digest::consts::B0>, hmac::digest::consts::B0>, hmac::digest::consts::B0>> = hmac.finalize().into_bytes();
    let numerator: u128 = u128::from_be_bytes(hash[..16].try_into().unwrap());
    let denominator: u128 = u128::MAX;
    if numerator > denominator {
        panic!("Numerator exceeds maximum value of u128.");
    }
    let mut result = numerator as f64 / denominator as f64;
    if result == 0.5 {
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        result = if random_number < 0.5 { 0.6 } else { 0.4 };
    }
    result
}
fn main() {
    let server_seed = generate_server_seed();
    let client_seed = generate_client_seed();
    let result: f64 = coinflip(&server_seed, &client_seed);
    let mut flipinfo = json!({
        "Server_Seed": server_seed.to_string(),
        "Client_Seed": client_seed,
        "Result": result.to_string(),
    });
    println!("{}", flipinfo);
}
