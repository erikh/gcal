use crate::{
    resources::ConferenceProperties,
    sendable::{QueryParams, Sendable},
};
use serde_derive::{Deserialize, Serialize};

/*
 * from: https://developers.google.com/calendar/api/v3/reference/calendars#resource
 */

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
    fn path(&self, _action: Option<String>) -> String {
        format!("calendars/{}", self.id)
    }

    fn query(&self) -> QueryParams {
        Default::default()
    }
}
