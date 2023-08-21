use crate::{
    client::Client,
    resources::DefaultReminder,
    sendable::{AdditionalProperties, QueryParams, Sendable},
};
use reqwest::Response;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeSet;

/*
 * from: https://developers.google.com/calendar/api/v3/reference/events#resource
 */

fn default_kind() -> String {
    "calendar#event".to_string()
}

fn default_true() -> Option<bool> {
    Some(true)
}

pub struct EventClient(Client);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    #[serde(rename = "extendedProperties")]
    pub extended_properties: EventExtendedProperties,
    pub gadget: EventGadget,
    #[serde(rename = "guestsCanInviteOthers", default = "default_true")]
    pub guests_invite_others: Option<bool>,
    #[serde(rename = "guestsCanModify")]
    pub guests_can_modify: Option<bool>,
    #[serde(rename = "guestsCanSeeOtherGuests", default = "default_true")]
    pub guests_can_see_other_guests: Option<bool>,
    #[serde(rename = "hangoutLink")]
    pub hangout_link: String,
    #[serde(rename = "htmlLink")]
    pub html_link: String,
    #[serde(rename = "iCalUID")]
    pub ical_uid: String,
    pub location: String,
    pub locked: bool,
    pub organizer: Option<EventOrganizer>,
    pub original_start_time: EventCalendarDate,
    #[serde(rename = "privateCopy")]
    pub private_copy: bool,
    pub recurrence: BTreeSet<String>,
    pub reminders: Option<EventReminder>,
    pub sequence: u64,
    pub source: Option<EventSource>,
    pub start: EventCalendarDate,
    pub status: EventStatus,
    pub summary: String,
    pub transparency: Option<EventTransparency>,
    pub updated: String,
    pub visibility: Option<EventVisibility>,
    pub working_location: Option<EventWorkingLocation>,
    #[serde(skip)]
    query_string: QueryParams,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventOfficeLocationType {
    #[default]
    #[serde(rename = "homeOffice")]
    HomeOffice,
    #[serde(rename = "officeLocation")]
    OfficeLocation,
    #[serde(rename = "customLocation")]
    CustomLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventOfficeLocation {
    #[serde(rename = "buildingId")]
    pub building_id: Option<String>,
    #[serde(rename = "deskId")]
    pub desk_id: Option<String>,
    #[serde(rename = "floorId")]
    pub floor_id: Option<String>,
    #[serde(rename = "floorSectionId")]
    pub floor_section_id: Option<String>,
    pub label: Option<String>,
    pub typ: EventOfficeLocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventCustomLocation {
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventWorkingLocation {
    #[serde(rename = "customLocation")]
    pub custom_location: Option<EventCustomLocation>,
    #[serde(rename = "homeOffice")]
    pub home_office: Option<String>,
    pub office_location: Option<EventOfficeLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventVisibility {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "confidential")]
    Confidential,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventTransparency {
    #[default]
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "transparent")]
    Transparent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[default]
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventSource {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventReminder {
    pub overrides: Vec<DefaultReminder>,
    #[serde(rename = "useDefault")]
    pub use_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventOrganizer {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: String,
    pub id: String,
    #[serde(rename = "self")]
    pub appears_as_self: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventGadgetDisplay {
    #[default]
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "chip")]
    Chip,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventGadget {
    pub display: EventGadgetDisplay,
    pub preferences: AdditionalProperties,
    // a lot of deprecated fields in this struct
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventExtendedProperties {
    pub private: AdditionalProperties,
    pub shared: AdditionalProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "outOfOffice")]
    OutOfOffice,
    #[serde(rename = "focusTime")]
    FocusTime,
    #[serde(rename = "workingLocation")]
    WorkingLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventCalendarDate {
    pub date: Option<String>,
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventCreator {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "self")]
    pub appears_as_self: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventConferenceEntryPointType {
    #[default]
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "sip")]
    SIP,
    #[serde(rename = "more")]
    More,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventCreateConferenceRequest {
    #[serde(rename = "conferenceSolutionKey")]
    conference_solution_key: EventConferenceSolutionKey,
    #[serde(rename = "requestId")]
    request_id: String,
    status: EventConferenceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventConferenceStatus {
    #[serde(rename = "statusCode")]
    status_code: EventConferenceStatusCode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventConferenceStatusCode {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventConferenceSolution {
    #[serde(rename = "iconUri")]
    pub icon_uri: String,
    pub key: EventConferenceSolutionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventConferenceSolutionKey {
    #[serde(rename = "type")]
    pub typ: EventConferenceSolutionKeyType,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventConferenceSolutionKeyType {
    EventHangout,
    #[serde(rename = "eventNamedHangout")]
    EventNamedHangout,
    #[default]
    #[serde(rename = "hangoutsMeet")]
    HangoutsMeet,
    #[serde(rename = "addOn")]
    AddOn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EventResponseStatus {
    #[default]
    #[serde(rename = "needsAction")]
    NeedsAction,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "tentative")]
    Tentative,
    #[serde(rename = "accepted")]
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    fn path(&self, action: Option<String>) -> String {
        format!(
            "calendars/{}/events/{}/{}",
            self.calendar_id,
            self.id,
            action.unwrap_or_default()
        )
    }

    fn query(&self) -> QueryParams {
        Default::default()
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
}
