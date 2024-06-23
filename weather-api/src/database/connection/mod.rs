use mongodb::{bson::doc, Client, Collection};
use no_connection_error::NoConnectionError;
use sensor_data::SensorData;
use std::{env, error::Error};

pub mod sensor_data;
pub mod no_connection_error;

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
        let url = env::var("MONGO_URL").expect("MONGO_URL not set");
        println!("Connecting to MongoDB at {}", url);
        let client = match Client::with_uri_str(url).await {
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
