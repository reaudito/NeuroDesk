use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}
