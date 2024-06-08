use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SensorData {
    sensor_id: String,
    timestamp: DateTime<Utc>,
    temperature: Option<f32>,
    humidity: Option<f32>,
    light_level: Option<f32>,
}
