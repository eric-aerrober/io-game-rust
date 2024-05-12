use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentitfierSource {
    Guest
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub display_name: String,
    pub source: IdentitfierSource
}

impl Identity {
    pub fn new(display_name: String, source: IdentitfierSource) -> Identity {
        Identity {
            display_name,
            source
        }
    }

    pub fn unknown() -> Identity {
        Identity {
            display_name: "Unknown".to_string(),
            source: IdentitfierSource::Guest
        }
    }
}