use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceProperties {
    #[serde(rename = "allowedConferenceSolutionTypes")]
    pub allowed_solution_types: Vec<AllowedSolutionType>,
    pub default_reminders: Vec<DefaultReminder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultReminder {
    pub method: ReminderMethod,
    pub minutes: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReminderMethod {
    #[serde(rename = "email")]
    EMail,
    #[serde(rename = "popup")]
    PopUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AllowedSolutionType {
    EventHangout,
    EventNamedHangout,
    HangoutsMeet,
}
