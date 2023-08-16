use std::collections::BTreeMap;
use url::Url;

const BASE_URL: &str = "https://www.googleapis.com/calendar/v3/users/me/";

pub trait Sendable
where
    Self: serde::Serialize,
{
    fn path(&self) -> String;
    fn query(&self) -> BTreeMap<String, String>;

    fn url(&self) -> Result<Url, anyhow::Error> {
        Ok(Url::parse_with_params(
            &format!("{}{}", BASE_URL, self.path()),
            self.query(),
        )?)
    }

    fn body_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(serde_json::to_vec(self)?)
    }
}
