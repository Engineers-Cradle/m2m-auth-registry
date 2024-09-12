use std::fs;
use jwtk::{self};
use chrono::Duration;

pub fn read_private_key() -> jwtk::ecdsa::EcdsaPrivateKey{
    let private_key_content = fs::read_to_string("keys/private.pem").unwrap();
    let private_key = jwtk::ecdsa::EcdsaPrivateKey::from_pem(
        private_key_content.as_bytes(),
    ).unwrap();
    private_key
}

pub fn generate_jwt(node_id: String, app_node: String, private_key: &jwtk::ecdsa::EcdsaPrivateKey) -> String {
    let mut header_and_claims: jwtk::HeaderAndClaims<serde_json::Map<String, serde_json::Value>> = jwtk::HeaderAndClaims::new_dynamic();
    header_and_claims.set_sub("m2m-service");
    header_and_claims.set_iss("auth-registry");
    header_and_claims.set_iat_now();
    header_and_claims.set_nbf_from_now(std::time::Duration::from_secs(0));
    header_and_claims.set_exp_from_now(std::time::Duration::from_secs(Duration::hours(2).num_seconds() as u64));
    header_and_claims.insert("node_id", node_id);
    header_and_claims.insert("app_node", app_node);
    
    let jwt = jwtk::sign(&mut header_and_claims, private_key).unwrap();
    jwt
}