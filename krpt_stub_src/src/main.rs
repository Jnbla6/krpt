use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyInit};
use sha2::{Digest, Sha256};

use std::env;
use std::fs::File;
use std::io::{Read, Write};


type Aes256EcbDec = ecb::Decryptor<aes::Aes256>;


fn decrypt_bytes(cipherbytes: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let dec = Aes256EcbDec::new(key.into());

    let mut buffer = cipherbytes.to_vec();

    let plain_len = dec.decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .expect("Decryption failed Incorrect password or corrupted data.")
        .len();

    buffer.truncate(plain_len);
    buffer
}


fn hashingvec(pass: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&pass);
    let key: [u8; 32] = hasher.finalize().into();
    key
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: Password or payload path argument missing.");
        std::process::exit(1);
    }

    let pass = &args[1];
    let payload_path = &args[2];

    let mut payloadfile = File::open(payload_path)
        .expect("Failed to open payload file");

    let mut payload = Vec::new();

    payloadfile.read_to_end(&mut payload)
        .expect("Failed to open payload file");


    if payload.len() < 72 {
        eprintln!("Error: File is too small. No payload found.");
        std::process::exit(1);
    }


    let payload_size_bytes: [u8; 8] = payload[payload.len() - 8..]
        .try_into()
        .unwrap();

    let payload_len = u64::from_le_bytes(payload_size_bytes) as usize;


    let payload_start = payload.len()
    .wrapping_sub(payload_len)
    .wrapping_sub(72);

    if payload_start > payload.len() {
        eprintln!("Error: Invalid payload size metadata.");
        std::process::exit(1);
    }

    let salt_start = payload_start;


    let encrypted_payload_start = payload.len()
        .wrapping_sub(payload_len)
        .wrapping_sub(56);


    let encrypted_payload_end = payload.len() - 56;
    let encrypted_main_key_end = payload.len() - 8;

    let salt = &payload[
        salt_start..encrypted_payload_start
    ];

    let encrypted_payload = &payload[
        encrypted_payload_start..encrypted_payload_end
    ];

    let encrypted_main_key = &payload[
        encrypted_payload_end..encrypted_main_key_end
    ];


    let mut firstline = Vec::new();

    firstline.extend_from_slice(pass.as_bytes());
    firstline.extend_from_slice(salt);

    let hashedkey = hashingvec(firstline);


    let decryptedmainkey = decrypt_bytes(
        encrypted_main_key,
        &hashedkey
    );

    let mainkey: [u8; 32] = decryptedmainkey
        .try_into()
        .unwrap();


    let mut decryptedfile = File::create("decrypted_file.out")
        .expect("Failed to create output file");


    let mut buffer = [0u8; 1040];

    let mut encryptedfile = std::io::Cursor::new(
        encrypted_payload
    );


    loop {
        let byte_read = encryptedfile
            .read(&mut buffer)
            .unwrap();

        if byte_read == 0 {
            break;
        }

        let plain_bytes = decrypt_bytes(
            &buffer[..byte_read],
            &mainkey
        );

        decryptedfile
            .write_all(&plain_bytes)
            .expect("Decryption failed!");
    }


    println!(
        "Success! File decrypted successfully as: decrypted_file.out"
    );
}
