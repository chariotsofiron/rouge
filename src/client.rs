use crate::models::auth::Auth;
use crate::Request;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{self, StatusCode};
use serde::de::DeserializeOwned;
use url::Url;

async fn handle_response<T: DeserializeOwned>(
    res: reqwest::Response,
) -> Result<T, Box<dyn std::error::Error>> {
    match res.status() {
        StatusCode::OK => {
            let bytes = res.bytes().await?;
            match serde_json::from_slice::<T>(&bytes) {
                Ok(response) => Ok(response),
                Err(z) => {
                    println!("{}", String::from_utf8(bytes.into_iter().collect())?);
                    Err(z.into())
                }
            }
        }
        s => Err(format!("{}", s).into()),
    }
}

pub struct Client {
    client: reqwest::Client,
    auth: Auth,
}

impl Client {
    pub async fn new(user_agent: &str, auth: Auth) -> Result<Self, Box<dyn std::error::Error>> {
        let mut url = Url::parse("https://www.reddit.com")?.join(&auth.path())?;

        let (client_id, secret_id) = match &auth {
            Auth::None => {
                return Ok(Client {
                    client: reqwest::Client::new(),
                    auth,
                });
            }
            Auth::Password {
                client_id,
                secret_id,
                ..
            } => (client_id, secret_id),
            Auth::ClientCredentials {
                client_id,
                secret_id,
            } => (client_id, secret_id),
        };

        url.set_query(Some(&serde_urlencoded::to_string(&auth)?));
        let response = reqwest::Client::new()
            .post(url)
            .basic_auth(client_id, Some(secret_id))
            .send()
            .await?;

        let token: <Auth as Request>::Response = handle_response(response).await?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap(),
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .user_agent(user_agent)
            .build()?;

        Ok(Client { client, auth })
    }

    pub async fn request<R>(&self, req: R) -> Result<R::Response, Box<dyn std::error::Error>>
    where
        R: Request,
    {
        if R::REQUIRES_USER && !matches!(self.auth, Auth::Password { .. }) {
            return Err("This API resource requires authentication using `Auth::Password`".into());
        }
        let mut url = match &self.auth {
            Auth::None => {
                Url::parse("https://www.reddit.com")?.join(&format!("{}/.json", req.path()))?
            }
            Auth::ClientCredentials { .. } | Auth::Password { .. } => {
                Url::parse("https://oauth.reddit.com")?.join(&req.path())?
            }
        };

        url.set_query(Some(&serde_urlencoded::to_string(&req)?));
        let res = self.client.request(R::METHOD, url).send().await?;
        handle_response(res).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Auth, Client};
    use crate::models::inbox::Inbox;
    use crate::models::{flatten, me::Me, user::Comments};

    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        let user_agent = "macos:mybot:0.0.1 (by /u/myusername)";
        let client = Client::new(user_agent, Auth::None).await?;
        let req = Comments {
            username: "spez".to_string(),
        };

        let req = Inbox {
            after: "".to_string(),
            before: "".to_string(),
            count: 5,
            limit: 25,
        };

        let resp = client.request(req).await.unwrap();
        // let comments = flatten(resp);
        // println!("{:?}", comments);
        Ok(())
    }
}
