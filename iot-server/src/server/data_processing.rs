use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::{error::Error};

#[derive(Serialize, Deserialize)]
pub struct SensorData {
    sensor_id: String,
    timestamp: String,
    temperature: Option<f32>,
    humidity: Option<f32>,
    light_level: Option<f32>,
}


pub struct DataProcessor {
    collection: Option<Collection<SensorData>>,
    cache_file: File
}

impl DataProcessor {
    pub async fn new() -> Result<DataProcessor, Box<dyn Error>> {
        Ok(DataProcessor { collection: DataProcessor::connect().await, cache_file: File::create("cache.json")? })
    }

    async fn connect() -> Option<Collection<SensorData>> {
        let client = match Client::with_uri_str("mongodb://localhost:27017").await {
            Ok(client) => client,
            Err(_) => return None
        };

        let db = client.database("IoT-DB");
        if db.run_command(doc! {"create": "SensorData"}, None).await.is_err() {
            println!("[*]Tables already exist")
        }
        Some(db.collection("SensorData"))
    }


    pub async fn insert(&mut self, data: SensorData) -> Result<(), Box<dyn Error>> {
        if let Some(collection) = &self.collection {
            todo!("Add inserting of json cache here");
            if let Err(err) = self.insert_mongodb(&collection, data).await {
                self.collection = None;
                return Err(err)
            }
            Ok(())
        } else {
            self.collection = DataProcessor::connect().await;
            self.insert_cache(data)
        }
    }

    async fn insert_mongodb(&self, collection: &Collection<SensorData>, data: SensorData) -> Result<(), Box<dyn Error>> {
        collection.insert_one(data, None)
        .await
        .map(|_| ())
        .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn insert_cache(&self, data: SensorData) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&data)?;
        writeln!(self.cache_file, "{}", json)?;
        Ok(())
    }

    fn send_cache_to_db(&self) {
        let buf;
        self.cache_file.read_to_end(buf);
        
        todo!("Implement sending of cache")
    }
}
