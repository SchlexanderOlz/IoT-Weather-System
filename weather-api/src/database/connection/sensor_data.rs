use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SensorData {
    sensor_id: String,
    timestamp: DateTime<Utc>,
    temperature: Option<f32>,
    humidity: Option<u8>,
    light_level: Option<f32>,
    pressure: Option<u32>,
}
