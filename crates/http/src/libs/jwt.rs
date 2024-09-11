use core::str;

use min_jwt::encode_and_sign;
use p256::ecdsa;
use serde_json::json;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Header {
    alg: String,
    typ: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    sub: String,
    name: String,
    iat: i64,
}

pub fn generate_keys() -> (ecdsa::SigningKey, ecdsa::VerifyingKey) {
    let private_key = ecdsa::SigningKey::random(&mut rand::thread_rng());
    let public_key = ecdsa::VerifyingKey::from(&private_key);
    (private_key, public_key)
}

pub fn generate_jwt(node_id: String, app_node: String, private_key: &ecdsa::SigningKey) -> String {
    let binding = serde_json::to_value(json!({
        "alg": "ES256",
        "typ": "JWT",
    }))
    .unwrap()
    .to_string();

    let header = binding.as_bytes();

    let binding = serde_json::to_value(json!({
        "sub": node_id,
        "name": app_node,
        "iat": chrono::Utc::now().timestamp(),
    }))
    .unwrap()
    .to_string();

    let claims = binding.as_bytes();

    let jwt = encode_and_sign(header, claims, &private_key).unwrap();
    jwt
}

pub fn verify_jwt(jwt: &str, public_key: &ecdsa::VerifyingKey) -> bool {
    let jwt = match min_jwt::verify(jwt, public_key) {
        Ok(jwt) => jwt,
        Err(_) => return false,
    };

    let header: Vec<u8> = jwt.decode_header().unwrap();
    let header = str::from_utf8(&header).unwrap();
    let header: Header = serde_json::from_str(header).unwrap();
    let claims: Vec<u8> = jwt.decode_claims().unwrap();
    let claims = str::from_utf8(&claims).unwrap();
    let claims: Claims = serde_json::from_str(claims).unwrap();

    if header.alg != "ES256" {
        return false;
    }

    true
}
