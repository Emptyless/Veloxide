#[derive(thiserror::Error, Debug)]
pub enum CryptograhyError {
    #[error("failed to create hmac from key")]
    FailedToCreateHmacFromKey,
}
