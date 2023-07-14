use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    name: String,
    timestamp: DateTime<Utc>,
}
