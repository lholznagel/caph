use crate::error::Error;

use hmac::{Hmac, Mac};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use sha2::Sha256;

/// Generates a secure token that can be used as a cookie token
/// 
/// # Errors
/// 
/// When either the ENV `SECRET_KEY` is not available or if generating
/// the HMAC key fails.
/// 
/// # Returns
/// 
/// Base64 encoded key and a base64 encoded hashed version of the key
/// 
pub fn generate_secure_token() -> Result<(String, String), Error> {
    let secret = std::env::var("SECRET_KEY")
        .map_err(Error::MissingEnvSecretKey)?;

    let mut rng = ChaCha20Rng::from_entropy();
    let mut key: Vec<u8> = (0..255).collect::<Vec<_>>();
    key.shuffle(&mut rng);
    let token = base64::encode(key.clone());

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(Error::HmacInitError)?;
    mac.update(&key);
    let result = mac.finalize();
    let result = result.into_bytes();
    let hashed = base64::encode(result);
    Ok((token, hashed))
}

/// TODO: validate
pub fn recreate_secure_token(
    token: String,
) -> Result<String, Error> {
    let secret = std::env::var("SECRET_KEY")
        .map_err(Error::MissingEnvSecretKey)?;

    let token = base64::decode(token)
        .map_err(Error::InvalidBase64)?;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(Error::HmacInitError)?;
    mac.update(&token);

    let result = mac.finalize();
    let result = result.into_bytes();

    Ok(base64::encode(result))
}
