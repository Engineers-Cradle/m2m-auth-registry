use core::str;
use std::fs;
use jwtk::{self, PublicKeyToJwk};
use chrono::Duration;

pub fn read_private_key() -> jwtk::ecdsa::EcdsaPrivateKey{
    let private_key_content = fs::read_to_string("../../keys/private.pem").unwrap();
    let private_key = jwtk::ecdsa::EcdsaPrivateKey::from_pem(
        private_key_content.as_bytes(),
    ).unwrap();
    private_key
}

pub fn read_public_key() -> jwtk::ecdsa::EcdsaPublicKey {
    let private_key_content = fs::read_to_string("../../keys/public.pem").unwrap();
    let public_key = jwtk::ecdsa::EcdsaPublicKey::from_pem(
        private_key_content.as_bytes(),
    ).unwrap();

    public_key
}

pub fn generate_jwks(
    private_key: jwtk::ecdsa::EcdsaPrivateKey,
) -> jwtk::jwk::Jwk {
    let jwks = private_key.public_key_to_jwk().unwrap();

    jwks
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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VerifyNDecodedResult {
    pub success: bool,
    pub node_id: Option<String>,
    pub app_node: Option<String>,
}

pub fn verify_jwt(jwt: &str, public_key: jwtk::ecdsa::EcdsaPublicKey) -> VerifyNDecodedResult {
    let jwt = match jwtk::verify::<serde_json::Map<String, serde_json::Value>>(jwt, &public_key) {
        Ok(jwt) => jwt,
        Err(_) => return VerifyNDecodedResult {
            success: false,
            node_id: None,
            app_node: None,
        },
    };

    struct ExtraClaims {
        app_node: Option<String>,
        node_id: Option<String>,
    }

    #[allow(dead_code)]
    struct Claims {
        sub: Option<String>,
        iss: Option<String>,
        iat: Option<i64>,
        nbf: Option<i64>,
        exp: Option<i64>,
        aud: Option<Vec<String>>,
        jti: Option<String>,
        extra: ExtraClaims,
    }

    #[allow(dead_code)]
    struct Header {
        alg: String,
        typ: Option<String>,
        kid: Option<String>
    }
    
    let header = jwt.header();
    let claims = jwt.claims();

    #[allow(unused_variables)]
    let header = Header {
        alg: header.alg.to_string(),
        typ: header.typ.clone().map(|x| x.to_string()),
        kid: header.kid.clone().map(|x| x.to_string())
    };

    let claims = Claims {
        sub: claims.sub.clone().map(|x| x.to_string()),
        iss: claims.iss.clone().map(|x| x.to_string()),
        iat: claims.iat.map(|x| x.as_secs() as i64),
        nbf: claims.nbf.map(|x| x.as_secs() as i64),
        exp: claims.exp.map(|x| x.as_secs() as i64),
        aud: None,
        jti: claims.jti.clone().map(|x| x.to_string()),
        extra: ExtraClaims {
            app_node: claims.extra.get("app_node").map(|x| x.to_string()),
            node_id: claims.extra.get("node_id").map(|x| x.to_string()),
        }
    };

    let node_id = claims.extra.node_id.clone().map(|x| x.replace("\"", ""));
    let app_node = claims.extra.app_node.clone().map(|x| x.replace("\"", ""));

    VerifyNDecodedResult {
        success: true,
        node_id,
        app_node,
    }
}