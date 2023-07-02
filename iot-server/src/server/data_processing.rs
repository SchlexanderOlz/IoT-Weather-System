use mongodb::{Collection};
use std::error::Error;
use std::mem::take;
use db_connection::{DBConnection, sensor_data::SensorData, no_connection_error::NoConnectionError};
use async_trait::async_trait;

pub mod sensor_data;

const MIN_CACHE_SIZE: usize = 2000;

pub struct DataProcessor {
    connection: DBConnection,
    cache: Vec<SensorData>
}

impl DataProcessor {
    pub async fn new() -> Result<DataProcessor, Box<dyn Error>> {
        Ok(DataProcessor {
            connection: DBConnection::new().await?,
            cache: Vec::with_capacity(MIN_CACHE_SIZE),
        })
    }
}


#[async_trait]
pub trait Inserter {
    async fn insert(&mut self, data: Vec<SensorData>) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
impl Inserter for DataProcessor {
    async fn insert(&mut self, data: Vec<SensorData>) -> Result<(), Box<dyn Error>> {
        async fn insert_mongodb(
            collection: &Collection<SensorData>,
            data: Vec<SensorData>,
        ) -> Result<(), Box<dyn Error>> {
            collection
                .insert_many(data, None)
                .await
                .map(|_| ())
                .map_err(|err| Box::new(err) as Box<dyn Error>)
        }

        if let Some(collection) = &self.connection.get_collection() {
            if let Err(err) = insert_mongodb(collection, data.clone()).await {
                self.connection.reset();

                let mut data_copy = data.clone();
                self.cache.append(&mut data_copy);
                return Err(err);
            }
            if self.cache.is_empty() {
                return Ok(());
            }
            let cache = take(&mut self.cache);
            insert_mongodb(collection, cache).await?;
            Ok(())
        } else {
            self.connection.reconnect().await;
            let mut data_copy = data.clone();
            self.cache.append(&mut data_copy);
            Err(Box::new(NoConnectionError))
        }
    }
}