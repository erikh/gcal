pub mod calendar_list;
pub use calendar_list::*;
pub mod calendar;
pub use calendar::*;
pub mod conference_properties;
pub use conference_properties::*;
pub mod events;
pub use events::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum SendUpdates {
    #[default]
    #[serde(rename = "all")]
    All,
    #[serde(rename = "externalOnly")]
    ExternalOnly,
    #[serde(rename = "none")]
    None,
}

impl ToString for SendUpdates {
    fn to_string(&self) -> String {
        match self {
            Self::All => "all",
            Self::ExternalOnly => "externalOnly",
            Self::None => "none",
        }
        .to_string()
    }
}
