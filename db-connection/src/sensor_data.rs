use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::data::{Decoder, u32_iter_to_next, f32_iter_to_next, string_iter_to_next};


#[derive(Serialize, Deserialize)]
pub struct SensorData {
    sensor_id: String,
    timestamp: DateTime<Utc>,
    temperature: Option<f32>,
    humidity: Option<u8>,
    light_level: Option<f32>,
    pressure: Option<u32>,
}


impl Clone for SensorData {
    fn clone(&self) -> Self {
        Self { sensor_id: self.sensor_id.clone(), timestamp: self.timestamp.clone(), temperature: self.temperature, humidity: self.humidity, light_level: self.light_level, pressure: self.pressure }
    }
}

impl Decoder for SensorData {
    fn from_bytes(bytes: Vec<u8>) -> Self {

        let (mut sensor_id, mut temperature, mut humidity, mut light_level, mut pressure)
        = (None, None, None, None, None);


        let mut iter: std::slice::Iter<'_, u8> = bytes.iter();

        while let Some(byte) = iter.next() {
            match byte {
                0x1 => sensor_id = Some(string_iter_to_next(&mut iter)),
                0x2 => temperature = Some(f32_iter_to_next(&mut iter)),
                0x3 => humidity = Some(*iter.next().expect("Iterator was empty")),
                0x4 => light_level = Some(f32_iter_to_next(&mut iter)),
                0x5 => pressure = Some(u32_iter_to_next(&mut iter)),
                _ => ()
            }
        }

        let sensor_id = sensor_id.expect("Value was not initialized -> sensor_id value");
        Self { sensor_id, timestamp: Utc::now(), temperature, humidity, light_level, pressure }
    }
}

