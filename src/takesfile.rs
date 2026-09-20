// function to read the plain file then encrypt and write it to new file

use crate::encrypt::encrypt_bytes;
use std::io::{self, Read, Write};
use std::fs::File;

pub fn process(key: &[u8; 32], mut plainfile: File, cipherfile: &mut File) -> io::Result<()> {
    let mut buffer = [0u8; 1024];

    loop {
        let byte_read = plainfile.read(&mut buffer)?;

        if byte_read == 0 {
        break;
        }
        let cipher_bytes = encrypt_bytes(&buffer[..byte_read], key);
        cipherfile.write_all(&cipher_bytes)?;

    }

Ok(())
}
