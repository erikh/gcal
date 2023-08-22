use crate::sendable::Sendable;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, RequestBuilder, Response,
};

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    access_key: String,
    headers: Option<HeaderMap<HeaderValue>>,
}

impl Client {
    pub fn new(access_key: String) -> Result<Self, anyhow::Error> {
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

    async fn send(&self, mut req: RequestBuilder) -> Result<Response, anyhow::Error> {
        if let Some(headers) = &self.headers {
            req = req.headers(headers.clone())
        }

        Ok(self.set_bearer(req).send().await?.error_for_status()?)
    }

    pub async fn get(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, anyhow::Error> {
        self.send(self.client.get(target.url(action)?)).await
    }

    pub async fn post(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, anyhow::Error> {
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
    ) -> Result<Response, anyhow::Error> {
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
    ) -> Result<Response, anyhow::Error> {
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
    ) -> Result<Response, anyhow::Error> {
        self.send(self.client.delete(target.url(action)?)).await
    }
}
