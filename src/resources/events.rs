use crate::sendable::Sendable;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

/*
 * from: https://developers.google.com/calendar/api/v3/reference/events#resource
 */

fn default_kind() -> String {
    "calendar#event".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(default = "default_kind")]
    pub kind: String,
    pub id: String,
    pub calendar_id: String,
}

impl Sendable for Event {
    fn path(&self) -> String {
        format!("calendars/{}/events/{}", self.calendar_id, self.id)
    }

    fn query(&self) -> BTreeMap<String, String> {
        Default::default()
    }
}
