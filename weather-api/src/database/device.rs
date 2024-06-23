use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub timestamp: DateTime<Utc>,
}
