mod takesfile;
mod encrypt;

use std::fs::File;
use sha2::{Digest, Sha256};
use std::env;
use crate::takesfile::process;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_encrypt>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = format!("{}.krpt", input_path);

    let plainfile = File::open(input_path).expect("Failed to open input file");
    let cipherfile = File::create(&output_path).expect("Failed to create output file");


    let mut pass = String::new();

    println!("enter the main key to encrypt the file");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut pass).expect("failed to read the key");

    let mut hasher = Sha256::new();
    hasher.update(pass.trim().as_bytes());
    let key: [u8; 32] = hasher.finalize().into();
    
    process(&key, plainfile, cipherfile).expect("Failed during encryption process");
}
