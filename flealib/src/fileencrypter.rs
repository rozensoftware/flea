use aes_gcm::{aead::{Aead, Key, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use std::fs::File;
use std::io::{Read, Write};
use anyhow::Error;

pub struct FileEncrypter {
    key: Key<Aes256Gcm>,
}

impl FileEncrypter {
    pub fn new(key_str: String) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
        Self { key: *key }
    }

    pub fn encrypt_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        // Read the file
        let mut file = File::open(file_name)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        // Encrypt the file contents
        let encrypted_data = self.encrypt(String::from_utf8_lossy(&contents).to_string());

        // Overwrite the file with the encrypted data
        let mut file = File::create(file_name)?;
        file.write_all(&encrypted_data)?;

        Ok(())
    }

    pub fn decrypt_file(&self, file_name: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(file_name)?;
        let mut encrypted_data = Vec::new();
        file.read_to_end(&mut encrypted_data)?;

        match self.decrypt(encrypted_data)
        {
            Ok(decrypted_data) => 
            {
                let mut file = File::create(file_name)?;
                file.write_all(decrypted_data.as_bytes())?;
                Ok(())
            },
            Err(_) => 
            {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to decrypt"))
            }
        }
    }
    
    fn encrypt(&self, plaintext: String) -> Vec<u8> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(&self.key.clone());

        let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes())
            .expect("failed to encrypt");

        // Combine nonce and encrypted data together
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphered_data);
        result
    }

    fn decrypt(&self, ciphertext_with_nonce: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        // Split the nonce and ciphertext
        let nonce = Nonce::from_slice(&ciphertext_with_nonce[..12]);
        let ciphertext = &ciphertext_with_nonce[12..];

        let cipher = Aes256Gcm::new(&self.key.clone());

        let txt = cipher.decrypt(nonce, ciphertext).map_err(Error::msg)?;
        Ok(String::from_utf8(txt).unwrap())
    }        
}
