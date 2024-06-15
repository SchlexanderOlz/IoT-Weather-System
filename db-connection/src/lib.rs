use mongodb::{bson::doc, Client, Collection};
use no_connection_error::NoConnectionError;
use sensor_data::SensorData;
use std::error::Error;

pub mod data;
pub mod no_connection_error;
pub mod sensor_data;

const MONGO_DB_URL: &str = "mongodb://127.0.0.1:27017";
const DATABASE_NAME: &str = "IoT-DB";

pub struct DBConnection {
    collection: Option<Collection<SensorData>>,
}

impl Clone for DBConnection {
    fn clone(&self) -> Self {
        Self {
            collection: self.collection.clone(),
        }
    }
}

impl DBConnection {

    #[inline]
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            collection: Self::connect().await,
        })
    }

    #[inline]
    pub fn reset(&mut self) {
        self.collection = None
    }

    #[inline]
    pub fn collection(&self) -> Result<&Collection<SensorData>, Box<dyn Error + Send + Sync>> {
        self.collection.as_ref().ok_or(Box::new(NoConnectionError))
    }


    #[inline]
    pub fn collection_mut(&mut self) -> Option<&mut Collection<SensorData>> {
        self.collection.as_mut()
    }

    #[inline]
    pub async fn reconnect(&mut self) {
        self.collection = Self::connect().await
    }

    pub async fn connect() -> Option<Collection<SensorData>> {
        let client = match Client::with_uri_str(MONGO_DB_URL).await {
            Ok(client) => client,
            Err(_) => return None,
        };

        let db = client.database(DATABASE_NAME);
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
