use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    fn new(sub: String) -> Self {
        Claims {
            sub,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
        }
    }
}

pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id.to_string());
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    encode(&header, &claims, &encoding_key)
}
