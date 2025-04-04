use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use std::fs::File;
use std::io::Write;
use std::env;
use dotenvy::from_filename;
use std::convert::TryInto;

fn main() {
    
    generate_keypair();
    from_filename(".env").ok();
    load_secret_key();

}

fn generate_keypair() {

    let pair = Keypair::new();
    let pub_key = pair.pubkey();
    let secrete_key = pair.to_base58_string();

    println!("Generated:");
    println!("Secrete key: {}", secrete_key);
    println!("Public key: {}", pub_key);
    
    save_secrete_key(pair.to_bytes());

}

fn save_secrete_key(secrete_keys_bytes: [u8; 64]) {

    let secrete_key_string = format!("SECRET_KEY=\"{:?}\"", secrete_keys_bytes);
    let mut file = File::create(".env").expect("Failed to create or open .env file");
    file.write_all(secrete_key_string.as_bytes()).expect("Failed to save secrete key to .env file");

}

fn load_secret_key() {
    let raw_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let bytes: Vec<u8> = raw_key
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid byte"))
        .collect();
    if bytes.len() != 64 {
        panic!("Expected 64 bytes, got {}", bytes.len());
    }

    let secret_key_array: [u8; 64] = bytes.try_into().expect("Failed to convert to [u8; 64]");
    let keypair = Keypair::from_bytes(&secret_key_array).expect("Invalid keypair bytes");

    let pub_key = keypair.pubkey();
    let secrete_key = keypair.to_base58_string();

    println!("\nLoaded:");
    println!("Secrete key: {}", secrete_key);
    println!("Public key: {}", pub_key);
}
