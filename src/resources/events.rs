use crate::{
    client::Client,
    resources::{CalendarAccessRole, DefaultReminder, SendUpdates},
    sendable::{AdditionalProperties, QueryParams, Sendable},
};
use reqwest::Response;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeSet;

/*
 * from: https://developers.google.com/calendar/api/v3/reference/events#resource
 */

fn default_event_kind() -> String {
    "calendar#event".to_string()
}

fn default_events_kind() -> String {
    "calendar#events".to_string()
}

fn default_true() -> Option<bool> {
    Some(true)
}

pub struct EventClient(Client);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    #[serde(default = "default_events_kind")]
    pub kind: String,
    pub etag: String,
    pub summary: String,
    pub description: String,
    pub updated: String,
    pub time_zone: String,
    pub access_role: CalendarAccessRole,
    pub default_reminders: Vec<DefaultReminder>,
    pub next_page_token: Option<String>,
    pub items: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(default = "default_event_kind")]
    pub kind: String,
    pub id: Option<String>,
    pub calendar_id: Option<String>,
    pub attachments: Option<Vec<EventAttachment>>,
    pub attendees_omitted: Option<bool>,
    pub attendees: Option<Vec<EventAttendees>>,
    pub color_id: Option<String>,
    pub conference_data: Option<EventConferenceData>,
    pub created: Option<String>,
    pub creator: Option<EventCreator>,
    pub description: Option<String>,
    pub end: Option<EventCalendarDate>,
    pub end_time_unspecified: Option<bool>,
    pub etag: String,
    pub event_type: Option<EventType>,
    pub extended_properties: Option<EventExtendedProperties>,
    pub gadget: Option<EventGadget>,
    #[serde(rename = "guestsCanInviteOthers", default = "default_true")]
    pub guests_invite_others: Option<bool>,
    pub guests_can_modify: Option<bool>,
    #[serde(default = "default_true")]
    pub guests_can_see_other_guests: Option<bool>,
    pub hangout_link: Option<String>,
    pub html_link: Option<String>,
    #[serde(rename = "iCalUID")]
    pub ical_uid: Option<String>,
    pub location: Option<String>,
    pub locked: Option<bool>,
    pub organizer: Option<EventOrganizer>,
    pub original_start_time: Option<EventCalendarDate>,
    pub private_copy: Option<bool>,
    pub recurrence: Option<BTreeSet<String>>,
    pub reminders: Option<EventReminder>,
    pub sequence: Option<u64>,
    pub source: Option<EventSource>,
    pub start: Option<EventCalendarDate>,
    pub status: EventStatus,
    pub summary: Option<String>,
    pub transparency: Option<EventTransparency>,
    pub updated: Option<String>,
    pub visibility: Option<EventVisibility>,
    pub working_location: Option<EventWorkingLocation>,
    #[serde(skip)]
    query_string: QueryParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventOfficeLocationType {
    #[default]
    HomeOffice,
    OfficeLocation,
    CustomLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventOfficeLocation {
    pub building_id: Option<String>,
    pub desk_id: Option<String>,
    pub floor_id: Option<String>,
    pub floor_section_id: Option<String>,
    pub label: Option<String>,
    pub typ: EventOfficeLocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCustomLocation {
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventWorkingLocation {
    pub custom_location: Option<EventCustomLocation>,
    pub home_office: Option<String>,
    pub office_location: Option<EventOfficeLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventVisibility {
    #[default]
    Default,
    Public,
    Private,
    Confidential,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventTransparency {
    #[default]
    Opaque,
    Transparent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventStatus {
    Confirmed,
    #[default]
    Tentative,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventSource {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventReminder {
    pub overrides: Option<Vec<DefaultReminder>>,
    pub use_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventOrganizer {
    pub display_name: Option<String>,
    pub email: String,
    pub id: Option<String>,
    #[serde(rename = "self")]
    pub appears_as_self: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventGadgetDisplay {
    #[default]
    Icon,
    Chip,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventGadget {
    pub display: EventGadgetDisplay,
    pub preferences: AdditionalProperties,
    // a lot of deprecated fields in this struct
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventExtendedProperties {
    pub private: AdditionalProperties,
    pub shared: Option<AdditionalProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    #[default]
    Default,
    OutOfOffice,
    FocusTime,
    WorkingLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCalendarDate {
    pub date: Option<String>,
    pub date_time: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceData {
    pub conference_id: Option<String>,
    pub conference_solution: Option<EventConferenceSolution>,
    pub create_request: Option<EventCreateConferenceRequest>,
    pub entry_points: Vec<EventConferenceEntryPoint>,
    pub notes: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCreator {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "self")]
    pub appears_as_self: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceEntryPoint {
    entry_point_type: EventConferenceEntryPointType,
    label: String,
    meeting_code: Option<String>,
    passcode: Option<String>,
    password: Option<String>,
    pin: Option<String>,
    uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventConferenceEntryPointType {
    #[default]
    Video,
    Phone,
    SIP,
    More,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCreateConferenceRequest {
    conference_solution_key: EventConferenceSolutionKey,
    request_id: String,
    status: EventConferenceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceStatus {
    status_code: EventConferenceStatusCode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventConferenceStatusCode {
    #[default]
    Pending,
    Success,
    Failure,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceSolution {
    pub icon_uri: String,
    pub key: EventConferenceSolutionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceSolutionKey {
    #[serde(rename = "type")]
    pub typ: EventConferenceSolutionKeyType,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventConferenceSolutionKeyType {
    EventHangout,
    EventNamedHangout,
    #[default]
    HangoutsMeet,
    AddOn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendees {
    pub additional_guests: Option<u8>,
    pub comment: Option<String>,
    pub display_name: Option<String>,
    pub email: String,
    pub id: Option<String>,
    pub optional: Option<bool>,
    pub organizer: Option<bool>,
    pub resource: Option<bool>,
    pub response_status: EventResponseStatus,
    #[serde(rename = "self")]
    pub appears_as_self: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EventResponseStatus {
    #[default]
    NeedsAction,
    Declined,
    Tentative,
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventAttachment {
    pub file_id: String,
    pub file_url: String,
    pub icon_link: String,
    pub mime_type: String,
    pub title: String,
}

impl Sendable for Event {
    fn path(&self, action: Option<String>) -> String {
        format!(
            "calendars/{}/events{}{}",
            self.calendar_id.clone().unwrap(),
            self.id
                .clone()
                .map_or_else(|| String::new(), |x| format!("/{}", x)),
            action.map_or_else(|| String::new(), |x| format!("/{}", x))
        )
    }

    fn query(&self) -> QueryParams {
        self.query_string.clone()
    }
}

impl EventClient {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn delete(&self, event: Event) -> Result<Response, anyhow::Error> {
        self.0.delete(None, event).await
    }

    pub async fn get(&self, id: String) -> Result<Event, anyhow::Error> {
        let resp = self.0.get(Some(id), Event::default()).await?;

        Ok(resp.json().await?)
    }

    pub async fn import(&self, event: Event) -> Result<Event, anyhow::Error> {
        let resp = self.0.post(Some("import".to_string()), event).await?;

        Ok(resp.json().await?)
    }

    pub async fn insert(
        &self,
        mut event: Event,
        send_updates: Option<SendUpdates>,
        max_attendees: Option<u8>,
    ) -> Result<Event, anyhow::Error> {
        if let Some(attachments) = event.attachments.clone() {
            if !attachments.is_empty() {
                event
                    .query_string
                    .insert("supportsAttachments".to_string(), "true".to_string());
            }
        }

        event.query_string.insert(
            "sendUpdates".to_string(),
            send_updates.map_or_else(|| "false".to_string(), |x| x.to_string()),
        );

        if let Some(ma) = max_attendees {
            event
                .query_string
                .insert("maxAttendees".to_string(), format!("{}", ma));
        }

        let resp = self.0.post(Some("import".to_string()), event).await?;

        Ok(resp.json().await?)
    }

    pub async fn instances(&self, event: Event) -> Result<Vec<Event>, anyhow::Error> {
        Ok(self
            .0
            .get(Some("instances".to_string()), event)
            .await?
            .json()
            .await?)
    }

    pub async fn list(
        &self,
        calendar_id: String,
        start_time: chrono::DateTime<chrono::Local>,
        end_time: chrono::DateTime<chrono::Local>,
    ) -> Result<Vec<Event>, anyhow::Error> {
        let mut event = Event::default();
        event
            .query_string
            .insert("timeMin".to_string(), start_time.to_rfc3339());
        event
            .query_string
            .insert("timeMax".to_string(), end_time.to_rfc3339());
        event.calendar_id = Some(calendar_id);

        let resp = self.0.get(None, event).await?;
        let text = resp.text().await?;
        eprintln!("{}", text);
        Ok(serde_json::from_str::<Events>(&text)?.items)
    }

    pub async fn move_to_calendar(
        &self,
        mut event: Event,
        destination: String,
        send_updates: Option<SendUpdates>,
    ) -> Result<(), anyhow::Error> {
        event
            .query_string
            .insert("destination".to_string(), destination);
        event.query_string.insert(
            "sendUpdates".to_string(),
            send_updates.map_or_else(|| "false".to_string(), |x| x.to_string()),
        );

        self.0.post(Some("move".to_string()), event).await?;
        Ok(())
    }

    pub async fn add(&self, text: String) -> Result<Event, anyhow::Error> {
        let mut event = Event::default();
        event.query_string.insert("text".to_string(), text);

        Ok(self
            .0
            .post(Some("quickAdd".to_string()), event)
            .await?
            .json()
            .await?)
    }

    pub async fn update(&self, event: Event) -> Result<Event, anyhow::Error> {
        Ok(self.0.put(None, event).await?.json().await?)
    }
}
