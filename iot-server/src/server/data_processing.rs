use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct SensorData {
    sensor_id: String,
    timestamp: String,
    temperature: Option<f32>,
    humidity: Option<f32>,
    light_level: Option<f32>,
}

pub struct DataProcessor {
    collection: Collection<SensorData>,
}

impl DataProcessor {
    pub async fn new() -> Result<DataProcessor, Box<dyn Error>> {
        let client = Client::with_uri_str("mongodb://localhost:27017").await?;
        let db = client.database("IoT-DB");
         db.run_command(doc! {"create": "SensorData"}, None)
            .await?;
        let collection = db.collection("SensorData");

        Ok(DataProcessor { collection })
    }

    pub async fn insert(&self, data: SensorData) -> Result<(), Box<dyn Error>> {
        self.collection.insert_one(data, None)
        .await
        .map(|_| ())
        .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}
