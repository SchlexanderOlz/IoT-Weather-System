use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::{error::Error};
use std::mem::take;


#[derive(Serialize, Deserialize)]
pub struct SensorData {
    sensor_id: String,
    timestamp: String,
    temperature: Option<f32>,
    humidity: Option<f32>,
    light_level: Option<f32>,
}

impl Clone for SensorData {
    fn clone(&self) -> Self {
        Self { sensor_id: self.sensor_id.clone(), timestamp: self.timestamp.clone(), temperature: self.temperature, humidity: self.humidity, light_level: self.light_level }
    }
}


const MIN_CACHE_SIZE: usize = 2000;
const MONGO_DB_URL: &str = "mongodb://192.168.8.127:27017";

pub struct DataProcessor {
    collection: Option<Collection<SensorData>>,
    cache: Vec<SensorData>
}


impl DataProcessor {
    pub async fn new() -> Result<DataProcessor, Box<dyn Error>> {
        Ok(DataProcessor { collection: DataProcessor::connect().await, cache: Vec::with_capacity(MIN_CACHE_SIZE) })
    }

    async fn connect() -> Option<Collection<SensorData>> {
        let client = match Client::with_uri_str(MONGO_DB_URL).await {
            Ok(client) => client,
            Err(_) => return None
        };

        let db = client.database("IoT-DB");
        if db.run_command(doc! {"create": "SensorData"}, None).await.is_err() {
            println!("[*]Tables already exist")
        }
        Some(db.collection("SensorData"))
    }


    pub async fn insert(&mut self, data: Vec<SensorData>) -> Result<(), Box<dyn Error>> {
        async fn insert_mongodb(collection: &Collection<SensorData>, data: Vec<SensorData>) -> Result<(), Box<dyn Error>> {
            collection.insert_many(data, None)
            .await
            .map(|_| ())
            .map_err(|err| Box::new(err) as Box<dyn Error>)
        }

        if let Some(collection) = &self.collection {
            if let Err(err) = insert_mongodb(&collection, data.clone()).await {
                self.collection = None;

                let mut data_copy = data.clone();
                self.cache.append(&mut data_copy);
                return Err(err)
            }
            if self.cache.is_empty() {
                return Ok(())
            }
            let cache = take(&mut self.cache);
            insert_mongodb(collection, cache).await?;
            Ok(())
        } else {
            self.collection = DataProcessor::connect().await;
            let mut data_copy = data.clone();
            self.cache.append(&mut data_copy);
            Ok(())
        }
    }


}
