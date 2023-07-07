use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize)]
pub struct SensorData {
    sensor_id: String,
    timestamp: DateTime<Utc>,
    temperature: Option<f32>,
    humidity: Option<u8>,
    light_level: Option<u32>,
    pressure: Option<u32>,
}


impl Clone for SensorData {
    fn clone(&self) -> Self {
        Self { sensor_id: self.sensor_id.clone(), timestamp: self.timestamp.clone(), temperature: self.temperature, humidity: self.humidity, light_level: self.light_level, pressure: self.pressure }
    }
}