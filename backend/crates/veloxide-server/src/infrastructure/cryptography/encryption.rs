use super::encoding::*;
use super::error::CryptograhyError;
use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha512;

pub struct EncryptionContent {
    pub content: String, // Clear content.
    pub salt: String,    // Clear salt.
}

/// Create a token signature with token parts
#[tracing::instrument(ret, err, level = "debug", skip(key))]
pub fn token_sign_into_base64url(
    ident: &str,
    exp: &str,
    salt: &str,
    key: &[u8],
) -> Result<String, CryptograhyError> {
    let content = format!(
        "{}.{}",
        base64urlsafe_encode(ident),
        base64urlsafe_encode(exp)
    );
    let signature = encrypt_content_to_base64_urlsafe(
        key,
        &EncryptionContent {
            content,
            salt: salt.to_string(),
        },
    )?;

    Ok(signature)
}

/// Encrypts content using HMAC-SHA512, then encodes the result into a Base64 URL-safe string.
///
/// This function takes a secret key and an `EncryptionContent` object as input.
/// The `EncryptionContent` includes the content and a salt. Both are converted into bytes and used to update
/// the HMAC-SHA512 instance. The HMAC result is then finalized and encoded into a Base64 URL-safe string.
///
/// # Arguments
///
/// * `key`: A byte slice representing the secret key used for HMAC.
/// * `enc_content`: An `EncryptionContent` object containing the content and salt to be encrypted.
///
pub fn encrypt_content_to_base64_urlsafe(
    key: &[u8],
    enc_content: &EncryptionContent,
) -> Result<String, CryptograhyError> {
    let EncryptionContent { content, salt } = enc_content;

    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)
        .map_err(|_| CryptograhyError::FailedToCreateHmacFromKey)?;

    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = general_purpose::URL_SAFE_NO_PAD.encode(result_bytes);

    Ok(result)
}
