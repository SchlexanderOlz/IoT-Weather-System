use mongodb::{bson::doc, Client, Collection};
use sensor_data::SensorData;
use std::error::Error;

pub mod sensor_data;
pub mod data;
pub mod no_connection_error;

const MONGO_DB_URL: &str = "mongodb://192.168.8.127:27017";

pub struct DBConnection {
    collection: Option<Collection<SensorData>>
}

impl Clone for DBConnection {
    fn clone(&self) -> Self {
        Self { collection: self.collection.clone() }
    }
}

impl DBConnection {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            collection: Self::connect().await
        })
    }

    pub fn reset(&mut self) { self.collection = None }

    pub fn get_collection(&self) -> Option<&Collection<SensorData>> { return self.collection.as_ref() }
    pub fn get_collection_mut(&mut self) -> Option<&mut Collection<SensorData>> { return self.collection.as_mut() }
    pub async fn reconnect(&mut self) {self.collection = Self::connect().await }


    pub async fn connect() -> Option<Collection<SensorData>> {
        let client = match Client::with_uri_str(MONGO_DB_URL).await {
            Ok(client) => client,
            Err(_) => return None,
        };

        let db = client.database("IoT-DB");
        if db
            .run_command(doc! {"create": "SensorData"}, None)
            .await
            .is_err()
        {
            println!("[*]Tables already exist")
        }
        Some(db.collection("SensorData"))
    }
}
