use crate::sendable::Sendable;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, RequestBuilder, Response,
};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ClientError {
    #[error("Invalid Access Token")]
    InvalidToken,
    #[error("Unknown Error: {0}")]
    UnknownError(String),
}

impl From<davisjr::errors::Error> for ClientError {
    fn from(value: davisjr::errors::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<anyhow::Error> for ClientError {
    fn from(value: anyhow::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(value: serde_json::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<url::ParseError> for ClientError {
    fn from(value: url::ParseError) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(value: reqwest::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<reqwest::header::ToStrError> for ClientError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        Self::UnknownError(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    access_key: String,
    headers: Option<HeaderMap<HeaderValue>>,
}

impl Client {
    pub fn new(access_key: String) -> Result<Self, ClientError> {
        let client = ClientBuilder::new().gzip(true).https_only(true).build()?;

        Ok(Self {
            client,
            access_key,
            headers: None,
        })
    }

    pub(crate) fn set_headers(&mut self, header: HeaderMap<HeaderValue>) {
        self.headers = Some(header)
    }

    fn set_bearer(&self, req: RequestBuilder) -> RequestBuilder {
        req.header("Authorization", format!("Bearer {}", self.access_key))
    }

    async fn send(&self, mut req: RequestBuilder) -> Result<Response, ClientError> {
        if let Some(headers) = &self.headers {
            req = req.headers(headers.clone())
        }

        let resp = self.set_bearer(req).send().await?;
        if resp.status() != 200 {
            if let Some(header) = resp.headers().get("WWW-Authenticate") {
                if header
                    .to_str()?
                    .starts_with(r#"Bearer error="invalid_token""#)
                {
                    return Err(ClientError::InvalidToken);
                }
            }

            Ok(resp.error_for_status()?)
        } else {
            Ok(resp)
        }
    }

    pub async fn get(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        self.send(self.client.get(target.url(action)?)).await
    }

    pub async fn post(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        self.send(
            self.client
                .post(target.url(action)?)
                .body(target.body_bytes()?),
        )
        .await
    }

    pub async fn put(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        self.send(
            self.client
                .put(target.url(action)?)
                .body(target.body_bytes()?),
        )
        .await
    }

    pub async fn patch(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        self.send(
            self.client
                .patch(target.url(action)?)
                .body(target.body_bytes()?),
        )
        .await
    }

    pub async fn delete(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        self.send(self.client.delete(target.url(action)?)).await
    }
}
