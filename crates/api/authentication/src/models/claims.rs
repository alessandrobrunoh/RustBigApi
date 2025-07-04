use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration timestamp
    pub iat: usize,  // Issued at timestamp
    pub iss: String, // Issuer
    pub aud: String, // Audience
    pub roles: Vec<String>, // User roles
}

