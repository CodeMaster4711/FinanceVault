use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("vault is locked")]
    Locked,
    #[error("vault not initialized")]
    NotInitialized,
    #[error("invalid passphrase")]
    InvalidPassphrase,
    #[error("invalid totp code")]
    InvalidTotp,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Serialize for VaultError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, VaultError>;
