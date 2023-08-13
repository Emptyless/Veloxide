use base64::{engine::general_purpose, Engine};

pub fn base64urlsafe_encode(content: &str) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content.as_bytes())
}

pub fn base64url_decode(b64u: &str) -> crate::prelude::Result<String> {
    let decoded_string = general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(crate::error::Error::Base64DecodeError)?;

    Ok(decoded_string)
}
