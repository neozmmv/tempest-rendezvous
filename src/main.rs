mod crypto;

fn main() {
    let message = "test message";
    let crypto = crypto::AEScrypt(message);
    println!("Encrypted message: {:?}", crypto.base64_encode());
    println!("Key: {:x?}", crypto.key);
    println!("Nonce: {:x?}", crypto.nonce);
    println!("Decrypted message: {}", crypto::AESdecrypt(&crypto));
    /* println!(
        "Decrypted message: {}",
        crypto::AESdecrypt(&crypto.message, &crypto.key, &crypto.nonce)
    ); */
    // assert_eq!(&plaintext, b"plaintext message");
    //println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));
    //println!("plaintext: {:?}", String::from_utf8_lossy(&plaintext));
    //println!("ok");
    //Ok(())
}
