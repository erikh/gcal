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
    pub attachments: Vec<EventAttachment>,
    #[serde(rename = "attendeesOmitted")]
    pub attendees_omitted: Option<bool>,
    pub attendees: Vec<EventAttendees>,
    #[serde(rename = "colorId")]
    pub color_id: Option<String>,
    #[serde(rename = "conferenceData")]
    pub conference_data: EventConferenceData,
    pub created: String,
    pub creator: EventCreator,
    pub description: Option<String>,
    pub end: EventCalendarDate,
    #[serde(rename = "endTimeUnspecified")]
    pub end_time_unspecified: bool,
    pub etag: String,
    pub event_type: EventType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "outOfOffice")]
    OutOfOffice,
    #[serde(rename = "focusTime")]
    FocusTime,
    #[serde(rename = "workingLocation")]
    WorkingLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCalendarDate {
    pub date: Option<String>,
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConferenceData {
    #[serde(rename = "conferenceId")]
    pub conference_id: Option<String>,
    #[serde(rename = "conferenceSolution")]
    pub conference_solution: Option<EventConferenceSolution>,
    #[serde(rename = "createRequest")]
    pub create_request: Option<EventCreateConferenceRequest>,
    #[serde(rename = "entryPoints")]
    pub entry_points: Vec<EventConferenceEntryPoint>,
    pub notes: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreator {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "self")]
    pub appears_as_self: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConferenceEntryPoint {
    #[serde(rename = "entryPointType")]
    entry_point_type: EventConferenceEntryPointType,
    label: String,
    #[serde(rename = "meetingCode")]
    meeting_code: Option<String>,
    passcode: Option<String>,
    password: Option<String>,
    pin: Option<String>,
    uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventConferenceEntryPointType {
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "sip")]
    SIP,
    #[serde(rename = "more")]
    More,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreateConferenceRequest {
    #[serde(rename = "conferenceSolutionKey")]
    conference_solution_key: EventConferenceSolutionKey,
    #[serde(rename = "requestId")]
    request_id: String,
    status: EventConferenceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConferenceStatus {
    #[serde(rename = "statusCode")]
    status_code: EventConferenceStatusCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventConferenceStatusCode {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConferenceSolution {
    #[serde(rename = "iconUri")]
    pub icon_uri: String,
    pub key: EventConferenceSolutionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConferenceSolutionKey {
    #[serde(rename = "type")]
    pub typ: EventConferenceSolutionKeyType,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventConferenceSolutionKeyType {
    #[serde(rename = "eventHangout")]
    EventHangout,
    #[serde(rename = "eventNamedHangout")]
    EventNamedHangout,
    #[serde(rename = "hangoutsMeet")]
    HangoutsMeet,
    #[serde(rename = "addOn")]
    AddOn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendees {
    #[serde(rename = "additionalGuests")]
    pub additional_guests: Option<u8>,
    pub comment: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub email: String,
    pub id: String,
    pub optional: Option<bool>,
    pub organizer: Option<bool>,
    pub resource: Option<bool>,
    #[serde(rename = "responseStatus")]
    pub response_status: EventResponseStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventResponseStatus {
    #[serde(rename = "needsAction")]
    NeedsAction,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "accepted")]
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttachment {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    #[serde(rename = "iconLink")]
    pub icon_link: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub title: String,
}

impl Sendable for Event {
    fn path(&self) -> String {
        format!("calendars/{}/events/{}", self.calendar_id, self.id)
    }

    fn query(&self) -> BTreeMap<String, String> {
        Default::default()
    }
}
