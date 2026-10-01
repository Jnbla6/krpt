mod takesfile;
mod encrypt;
mod hashing_keys;
mod stubinject;

use std::fs::File;
use std::env;
use crate::takesfile::process;
use crate::hashing_keys::hashingstring;
use crate::hashing_keys::hashingvec;
use crate::encrypt::encrypt_bytes;

use std::io::{self, Write};
use rand::{RngCore, rngs::OsRng};
use crate::stubinject::{stub_inject};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("no file to encrypt");
        eprintln!("Usage: krpt <file_to_encrypt>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = format!("{}.krpt", input_path);

    let mut frstpass = String::new();
    print!("> Enter the MAIN KEY (to encrypt the file payload): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut frstpass).expect("failed to read the key");
    let key = hashingstring(frstpass.trim().to_string());

    let mut secpass = String::new();
    println!();
    print!("> Enter the ACCESS KEY (to lock the Main Key): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut secpass).expect("failed to read the key");

    println!("  [!] IMPORTANT: The recipient only needs this ACCESS KEY to decrypt the file.");

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let firstline: Vec<u8> = [secpass.trim().as_bytes(), &salt].concat();
    let hashedkey = hashingvec(firstline);
    let injectedhashmainkey = encrypt_bytes(&key, &hashedkey);

    let plainfile = File::open(input_path).expect("Failed to open input file");

    let mut encrypted_payload = Vec::new();
    process(&key, plainfile, &mut encrypted_payload).unwrap();

    let payload_len = encrypted_payload.len() as u64;
    let payload_size_bytes = payload_len.to_le_bytes();

    let mut final_payload = Vec::new();
    final_payload.extend_from_slice(&salt);
    final_payload.extend_from_slice(&encrypted_payload);
    final_payload.extend_from_slice(&injectedhashmainkey);
    final_payload.extend_from_slice(&payload_size_bytes);

    let stub_bytes = include_bytes!("krpt_stub"); 

    println!("building the final file...");

    stub_inject(stub_bytes, &final_payload, &output_path);

}
