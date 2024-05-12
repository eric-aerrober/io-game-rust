use crate::networking::identity;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientIdentifySelf {
    pub display_name: String,
    pub source: identity::IdentitfierSource
}

// pub fn from_bytes(bytes: &[u8]) -> ClientIdentifySelf {
//     bincode::deserialize(bytes).unwrap()
// }

// pub fn to_bytes(client_identify_self: &ClientIdentifySelf) -> Vec<u8> {
//     bincode::serialize(client_identify_self).unwrap()
// }