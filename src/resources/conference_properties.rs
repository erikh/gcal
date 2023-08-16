use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConferenceProperties {
    #[serde(rename = "allowedConferenceSolutionTypes")]
    pub allowed_solution_types: Vec<AllowedSolutionType>,
    #[serde(rename = "defaultReminders")]
    pub default_reminders: Vec<DefaultReminder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultReminder {
    pub method: ReminderMethod,
    pub minutes: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReminderMethod {
    #[serde(rename = "email")]
    EMail,
    #[serde(rename = "popup")]
    PopUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedSolutionType {
    #[serde(rename = "eventHangout")]
    EventHangout,
    #[serde(rename = "eventNamedHangout")]
    EventNamedHangout,
    #[serde(rename = "hangoutsMeet")]
    HangoutsMeet,
}
