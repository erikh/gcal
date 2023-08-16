use crate::{resources::ConferenceProperties, sendable::Sendable};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

fn default_kind() -> String {
    "calendar#calendar".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    #[serde(default = "default_kind")]
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub summary: String,
    pub description: String,
    pub location: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "conferenceProperties")]
    pub conference_properties: ConferenceProperties,
}

impl Sendable for Calendar {
    fn path(&self) -> String {
        format!("calendars/{}", self.id)
    }

    fn query(&self) -> BTreeMap<String, String> {
        Default::default()
    }
}