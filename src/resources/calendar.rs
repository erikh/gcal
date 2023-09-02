use crate::{
    resources::ConferenceProperties,
    sendable::{QueryParams, Sendable},
};
use serde_derive::{Deserialize, Serialize};

/*
 * from: https://developers.google.com/calendar/api/v3/reference/calendars#resource
 */

fn default_kind() -> Option<String> {
    Some("calendar#calendar".to_string())
}

/// Calendar is a single calendar.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    #[serde(default = "default_kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub id: String,
    pub etag: String,
    pub summary: String,
    pub description: String,
    pub location: String,
    pub time_zone: String,
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
