use crate::client::Client;
use crate::resources::ConferenceProperties;
use crate::sendable::{QueryParams, Sendable};
use serde_derive::{Deserialize, Serialize};

/*
 * from: https://developers.google.com/calendar/api/v3/reference/calendarList#resource
 */

pub struct CalendarListClient(Client);

fn default_kind() -> String {
    "calendar#calendarListEntry".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarListItem {
    #[serde(default = "default_kind")]
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub location: Option<String>,
    pub summary: String,
    #[serde(rename = "summaryOverride")]
    pub summary_override: Option<String>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "accessRole")]
    pub access_role: CalendarAccessRole,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "foregroundColor")]
    pub foreground_color: Option<String>,
    #[serde(rename = "colorId")]
    pub color_id: Option<String>,
    #[serde(rename = "conferenceProperties")]
    pub conference_properties: Option<ConferenceProperties>,
    pub deleted: Option<bool>,
    pub hidden: Option<bool>,
    pub primary: Option<bool>,
    pub selected: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "notificationSettings")]
    pub notification_settings: NotificationSettings,
    #[serde(skip)]
    query_string: QueryParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub notifications: Vec<NotificationSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSetting {
    pub method: NotificationSettingMethod,
    #[serde(rename = "type")]
    pub typ: NotificationSettingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationSettingMethod {
    #[serde(rename = "email")]
    EMail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationSettingType {
    #[serde(rename = "eventCreation")]
    EventCreation,
    #[serde(rename = "eventChange")]
    EventChange,
    #[serde(rename = "eventCancellation")]
    EventCancellation,
    #[serde(rename = "eventResponse")]
    EventResponse,
    #[serde(rename = "agenda")]
    Agenda,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalendarAccessRole {
    #[serde(rename = "freeBusyReader")]
    FreeBusyReader,
    #[serde(rename = "reader")]
    Reader,
    #[serde(rename = "writer")]
    Writer,
    #[serde(rename = "owner")]
    Owner,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalendarList(QueryParams);

impl Sendable for CalendarListItem {
    fn path(&self, _action: Option<String>) -> String {
        format!("users/me/calendarList/{}", self.id)
    }

    fn query(&self) -> QueryParams {
        Default::default()
    }
}

impl Sendable for CalendarList {
    fn path(&self, _action: Option<String>) -> String {
        String::from("users/me/calendarList")
    }

    fn query(&self) -> QueryParams {
        self.0.clone()
    }
}

impl CalendarListClient {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn list(&self) -> Result<Vec<CalendarList>, anyhow::Error> {
        // FIXME get all the results lol
        let mut cl = CalendarList::default();
        cl.0.insert("minAccessRole".to_string(), "owner".to_string());
        Ok(self.0.get(None, cl).await?.json().await?)
    }
}
