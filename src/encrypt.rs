use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};

type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;

pub fn encrypt_bytes(plainbytes: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let enc = Aes256EcbEnc::new(key.into());


    let mut buffer = vec![0u8; plainbytes.len() + 16];

    buffer[..plainbytes.len()].copy_from_slice(plainbytes);

    let cipher_len = enc.encrypt_padded_mut::<Pkcs7>(&mut buffer, plainbytes.len())
        .expect("Encryption failed")
        .len();

    buffer.truncate(cipher_len);
    buffer
}
