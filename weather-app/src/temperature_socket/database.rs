use std::error::Error;
use mongodb::{options::{FindOneOptions}, bson::doc};
use db_connection::{DBConnection, sensor_data::SensorData, no_connection_error::NoConnectionError};
use no_data_error::NoDataError;
use tokio::runtime::Runtime;


mod no_data_error;

pub struct DataProcessor {
    connection: DBConnection
}

impl Clone for DataProcessor {
    fn clone(&self) -> Self {
        Self { connection: self.connection.clone() }
    }
}

impl DataProcessor {
    pub async fn new() -> Self {
        Self { connection: DBConnection::new().await.unwrap() }
    }
}

pub trait Selecter {
    fn get_newest_temperature(&self) -> Result<SensorData, Box<dyn Error>>;
}

impl Selecter for DataProcessor {
    fn get_newest_temperature(&self) -> Result<SensorData, Box<dyn Error>> {
        let options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();
    
        let collection = match self.connection.get_collection() {
            Some(collection) => collection,
            None => return Err(Box::new(NoConnectionError))
        };
    
        let rt = Runtime::new()?;
        let result = rt.block_on(async {
            collection.find_one(None, options).await
        })?;
    
        if let Some(result) = result {
            Ok(result)
        } else {
            Err(Box::new(NoDataError))
        }
    }
}