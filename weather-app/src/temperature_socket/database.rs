use std::error::Error;
use mongodb::{options::{FindOneOptions}, bson::doc};
use db_connection::{DBConnection, sensor_data::SensorData, no_connection_error::NoConnectionError};
use async_trait::async_trait;
use no_data_error::NoDataError;


mod no_data_error;

pub struct DataProcessor {
    connection: DBConnection
}

impl DataProcessor {
    pub async fn new() -> Self {
        Self { connection: DBConnection::new().await.unwrap() }
    }
}

#[async_trait]
pub trait Selecter {
    async fn get_newest_temperature(&self) -> Result<SensorData, Box<dyn Error>>;
}

#[async_trait]
impl Selecter for DataProcessor {
    async fn get_newest_temperature(&self) -> Result<SensorData, Box<dyn Error>> {
        let options = FindOneOptions::builder()
        .sort(doc! { "timestamp": -1 })
        .build();

        let collection = match self.connection.get_collection() {
            Some(collection) => collection,
            None => return Err(Box::new(NoConnectionError))
        };
        if let Some(result) = collection.find_one(None, options).await.unwrap() {
            Ok(result)
        } else {
            Err(Box::new(NoDataError))
        }
    }
}