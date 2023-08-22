use crate::client::Client;
use crate::resources::{CalendarAccessRole, ConferenceProperties};
use crate::sendable::{QueryParams, Sendable};
use crate::DefaultReminder;
use serde_derive::{Deserialize, Serialize};

/*
 * from: https://developers.google.com/calendar/api/v3/reference/calendarList#resource
 */

pub struct CalendarListClient(Client);

fn default_entry_kind() -> String {
    "calendar#calendarListEntry".to_string()
}

fn default_list_kind() -> String {
    "calendar#calendarList".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarListItem {
    #[serde(default = "default_entry_kind")]
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub location: Option<String>,
    pub summary: String,
    pub summary_override: Option<String>,
    pub time_zone: Option<String>,
    pub access_role: CalendarAccessRole,
    pub background_color: Option<String>,
    pub foreground_color: Option<String>,
    pub color_id: Option<String>,
    pub conference_properties: Option<ConferenceProperties>,
    pub deleted: Option<bool>,
    pub hidden: Option<bool>,
    pub primary: Option<bool>,
    pub selected: Option<bool>,
    pub description: Option<String>,
    pub notification_settings: NotificationSettings,
    pub default_reminders: Vec<DefaultReminder>,

    #[serde(skip)]
    query_string: QueryParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub notifications: Vec<NotificationSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSetting {
    pub method: NotificationSettingMethod,
    #[serde(rename = "type")]
    pub typ: NotificationSettingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationSettingMethod {
    #[serde(rename = "email")]
    EMail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationSettingType {
    EventCreation,
    EventChange,
    EventCancellation,
    EventResponse,
    Agenda,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CalendarList {
    #[serde(default = "default_entry_kind")]
    pub kind: String,
    pub etag: String,
    pub next_sync_token: String,
    pub items: Vec<CalendarListItem>,

    #[serde(skip)]
    query_string: QueryParams,
}

impl Sendable for CalendarListItem {
    fn path(&self, _action: Option<String>) -> String {
        format!("users/me/calendarList/{}", self.id)
    }

    fn query(&self) -> QueryParams {
        self.query_string.clone()
    }
}

impl Sendable for CalendarList {
    fn path(&self, _action: Option<String>) -> String {
        String::from("users/me/calendarList")
    }

    fn query(&self) -> QueryParams {
        self.query_string.clone()
    }
}

impl CalendarListClient {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn list(&self) -> Result<Vec<CalendarListItem>, anyhow::Error> {
        // FIXME get all the results lol
        let mut cl = CalendarList::default();
        cl.query_string
            .insert("minAccessRole".to_string(), "owner".to_string());

        let text = self.0.get(None, cl).await?.text().await?;
        eprintln!("{}", text);
        Ok(serde_json::from_str::<CalendarList>(&text)?.items)
    }
}
