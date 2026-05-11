use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelData {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

use tokio_util::sync::CancellationToken;
use tokio::sync::Mutex;

pub struct StreamState {
    pub cancel_token: Mutex<Option<CancellationToken>>,
}

impl Default for StreamState {
    fn default() -> Self {
        Self {
            cancel_token: Mutex::new(None),
        }
    }
}
