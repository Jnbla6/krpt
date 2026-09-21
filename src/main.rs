mod takesfile;
mod encrypt;
mod hashing_keys;

use std::fs::File;
use std::env;
use crate::takesfile::process;
use crate::hashing_keys::hashingstring;
use crate::hashing_keys::hashingvec;
use crate::encrypt::encrypt_bytes;

use std::io::{self, Write};
use rand::{RngCore, rngs::OsRng};
use base64::{engine::general_purpose, write::EncoderWriter};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("no file to encrypt");
        eprintln!("Usage: cargo run <file_to_encrypt>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = format!("{}.krpt", input_path);

    let mut frstpass = String::new();
    println!("enter the main key to encrypt the file");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut frstpass).expect("failed to read the key");
    let key = hashingstring(frstpass.trim().to_string());

    let mut secpass = String::new();
    println!("enter the second key to encrypt the main key");
    println!("note! that you will use this key to decrypt the file");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut secpass).expect("failed to read the key");

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let firstline: Vec<u8> = [secpass.trim().as_bytes(), &salt].concat();
    let hashedkey = hashingvec(firstline);
    let injectedhashmainkey = encrypt_bytes(&key, &hashedkey);

    let plainfile = File::open(input_path).expect("Failed to open input file");
    let mut cipherfile = File::create(&output_path).expect("Failed to create output file");

    let mut b64_writer = EncoderWriter::new(&mut cipherfile, &general_purpose::STANDARD);

    b64_writer.write_all(&salt).unwrap();
    process(&key, plainfile, &mut b64_writer).unwrap();
    b64_writer.write_all(&injectedhashmainkey).unwrap();

    b64_writer.finish().unwrap();


}
