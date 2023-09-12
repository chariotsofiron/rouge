use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::Request;

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}

#[derive(Serialize, PartialEq)]
#[serde(tag = "grant_type")]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    None,
    /// For API calls with a user context
    /// https://github.com/reddit-archive/reddit/wiki/OAuth2-Quick-Start-Example
    Password {
        #[serde(skip_serializing)]
        client_id: String,
        #[serde(skip_serializing)]
        secret_id: String,
        username: String,
        password: String,
    },
    /// For API calls without a user context
    /// https://github.com/reddit-archive/reddit/wiki/OAuth2#application-only-oauth
    ClientCredentials {
        #[serde(skip_serializing)]
        client_id: String,
        #[serde(skip_serializing)]
        secret_id: String,
    },
}
impl Request for Auth {
    const METHOD: Method = Method::POST;
    const PATH: &'static str = "/api/v1/access_token";
    const REQUIRES_USER: bool = true;

    type Response = LoginResponse;
}
