use crate::{
    client::{Client, ClientError},
    resources::{CalendarAccessRole, DefaultReminder, SendUpdates},
    sendable::{AdditionalProperties, QueryParams, Sendable},
};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeSet;

/*
 * from: https://developers.google.com/calendar/api/v3/reference/events#resource
 */

fn default_event_kind() -> Option<String> {
    Some("calendar#event".to_string())
}

fn default_events_kind() -> Option<String> {
    Some("calendar#events".to_string())
}

fn default_true() -> Option<bool> {
    Some(true)
}

pub struct EventClient(Client);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    #[serde(default = "default_events_kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub etag: String,
    pub summary: String,
    pub description: String,
    pub updated: String,
    pub time_zone: String,
    pub access_role: CalendarAccessRole,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_reminders: Vec<DefaultReminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(default = "default_event_kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EventAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees_omitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendees>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_data: Option<EventConferenceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<EventCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_unspecified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<EventExtendedProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gadget: Option<EventGadget>,
    #[serde(rename = "guestsCanInviteOthers", default = "default_true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_invite_others: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_modify: Option<bool>,
    #[serde(default = "default_true")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guests_can_see_other_guests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangout_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_link: Option<String>,
    #[serde(rename = "iCalUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ical_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<EventOrganizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<BTreeSet<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<EventReminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<EventSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<EventCalendarDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<EventTransparency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<EventVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_section_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_location: Option<EventCustomLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_office: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<DefaultReminder>>,
    pub use_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventOrganizer {
    pub display_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference_solution: Option<EventConferenceSolution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_request: Option<EventCreateConferenceRequest>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<EventConferenceEntryPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCreator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appears_as_self: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventConferenceEntryPoint {
    entry_point_type: EventConferenceEntryPointType,
    label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    meeting_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    passcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_guests: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<bool>,
    pub response_status: EventResponseStatus,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub async fn delete(&self, event: Event) -> Result<(), ClientError> {
        self.0.delete(None, event).await?;
        Ok(())
    }

    pub async fn get(&self, id: String) -> Result<Event, ClientError> {
        let resp = self.0.get(Some(id), Event::default()).await?;

        Ok(resp.json().await?)
    }

    pub async fn import(&self, event: Event) -> Result<Event, ClientError> {
        let resp = self.0.post(Some("import".to_string()), event).await?;

        Ok(resp.json().await?)
    }

    pub async fn insert(&self, mut event: Event) -> Result<Event, ClientError> {
        if let Some(attachments) = event.attachments.clone() {
            if !attachments.is_empty() {
                event
                    .query_string
                    .insert("supportsAttachments".to_string(), "true".to_string());
            }
        }

        let resp = self.0.post(Some("import".to_string()), event).await?;

        Ok(resp.json().await?)
    }

    pub async fn instances(&self, event: Event) -> Result<Events, ClientError> {
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
    ) -> Result<Vec<Event>, ClientError> {
        let mut event = Event::default();
        event
            .query_string
            .insert("timeMin".to_string(), start_time.to_rfc3339());
        event
            .query_string
            .insert("timeMax".to_string(), end_time.to_rfc3339());
        event.calendar_id = Some(calendar_id);

        Ok(self.0.get(None, event).await?.json::<Events>().await?.items)
    }

    pub async fn move_to_calendar(
        &self,
        mut event: Event,
        destination: String,
        send_updates: Option<SendUpdates>,
    ) -> Result<(), ClientError> {
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

    pub async fn add(&self, text: String) -> Result<Event, ClientError> {
        let mut event = Event::default();
        event.query_string.insert("text".to_string(), text);

        Ok(self
            .0
            .post(Some("quickAdd".to_string()), event)
            .await?
            .json()
            .await?)
    }

    pub async fn update(&self, event: Event) -> Result<Event, ClientError> {
        Ok(self.0.put(None, event).await?.json().await?)
    }
}
