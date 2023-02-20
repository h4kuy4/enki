use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};

pub struct Jwt {
    encode_key: EncodingKey,
    decode_key: DecodingKey,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            encode_key: EncodingKey::from_secret(secret.as_bytes()),
            decode_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn sign<T>(&self, claims: T) -> Result<String, String>
    where
        T: Serialize,
    {
        encode(&Header::default(), &claims, &self.encode_key).map_err(|err| err.to_string())
    }

    pub fn verify<T>(&self, token: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let token = decode::<T>(token, &self.decode_key, &Validation::default())
            .map_err(|err| format!("Invalid token: {}", err.to_string()))?;

        Ok(token.claims)
    }
}
