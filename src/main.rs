// https://blog.logrocket.com/making-http-requests-rust-reqwest/

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    scope: String,
    iss: String,
    iat: u64,
    exp: u64,
    jti: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    access_token: String,
    token_type: String,
    expires_in: u16,
    scope: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let integration_id = "c7e7e142-6527-4b90-8f23-faa9eee2fff4";
    let id = Uuid::new_v4();

    let aud = String::from("https://test.maskinporten.no/");
    let scope = "digdir:statistikk.skriv";
    let iss = integration_id.clone();
    let token_endpoint = aud.clone() + "token";

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(integration_id.to_owned());

    let unix_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("Aaargh, I died"),
    };

    let claim = Claims {
        aud: aud,
        scope: scope.to_string(),
        iss: iss.to_string(),
        iat: unix_time,
        exp: unix_time + 120,
        jti: id.to_string()
    };

    let token = encode(&header, &claim, &EncodingKey::from_rsa_pem(include_bytes!("keys.txt"))?)?;

    let mut params = HashMap::new();
    params.insert("assertion", &*token);
    params.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");

    let client = reqwest::Client::new();
    let response = client
        .post(token_endpoint)
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await {
                Ok(parsed) => println!("{}", serde_json::to_string(&parsed).unwrap()),
                Err(_) => println!("Response did not match")
            };
        }

        reqwest::StatusCode::BAD_REQUEST => {
            panic!("Invalid grant: {:?}", response.text().await.unwrap());
        }

        other => {
            panic!("Something went wrong: {:?}", other);
        }
    }

    Ok(())
}
