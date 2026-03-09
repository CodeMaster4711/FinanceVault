use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose::STANDARD, Engine};

use crate::error::{Result, VaultError};

pub const KEY_LEN: usize = 32;

pub struct DerivedKey(pub [u8; KEY_LEN]);

pub fn derive_key(passphrase: &str, salt_b64: &str) -> Result<DerivedKey> {
    let salt_bytes = STANDARD
        .decode(salt_b64)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;
    let salt = SaltString::encode_b64(&salt_bytes)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(passphrase.as_bytes(), &salt)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let hash_bytes = hash.hash.ok_or_else(|| VaultError::Crypto("no hash output".into()))?;
    let bytes = hash_bytes.as_bytes();

    if bytes.len() < KEY_LEN {
        return Err(VaultError::Crypto("derived key too short".into()));
    }

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&bytes[..KEY_LEN]);
    Ok(DerivedKey(key))
}

pub fn generate_salt_b64() -> String {
    let salt = SaltString::generate(&mut OsRng);
    STANDARD.encode(salt.as_str().as_bytes())
}

pub fn encrypt(key: &DerivedKey, plaintext: &[u8]) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.0));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(STANDARD.encode(&combined))
}

pub fn decrypt(key: &DerivedKey, encoded: &str) -> Result<Vec<u8>> {
    let combined = STANDARD
        .decode(encoded)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;

    if combined.len() < 12 {
        return Err(VaultError::Crypto("ciphertext too short".into()));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.0));
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| VaultError::InvalidPassphrase)
}
