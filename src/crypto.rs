use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use base64::{Engine as _, engine::general_purpose};

// this is a struct for an encrypted message
// THIS FILE WILL BE USED ON THE CLIENT, NOT THIS SERVER
// ITS HERE FOR TESTING PURPOSES
pub struct Crypto {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub message: Vec<u8>,
}

impl Crypto {
    pub fn base64_encode(&self) -> String {
        general_purpose::STANDARD.encode(&self.message)
    }

    pub fn get_key_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.key)
    }
}

#[allow(non_snake_case)]
pub fn AEScrypt(message: &str) -> Crypto {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // same as IV

    // message.as_ref() converts the string slice to a byte slice
    let ciphertext = cipher
        .encrypt(&nonce, message.as_ref())
        .expect("encryption failure!");
    Crypto {
        key: key.into(),
        nonce: nonce.into(),
        message: ciphertext,
    }
}

#[allow(non_snake_case)]
pub fn AESdecrypt(crypto: &Crypto) -> String {
    let cipher = Aes256Gcm::new_from_slice(&crypto.key).expect("invalid key length");
    let plaintext = cipher
        .decrypt(crypto.nonce.as_slice().into(), crypto.message.as_ref())
        .expect("decryption failure!");
    String::from_utf8_lossy(&plaintext).to_string()
}
