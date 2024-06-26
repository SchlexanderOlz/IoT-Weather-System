use crate::database::connection::{sensor_data::SensorData, DBConnection};
use actix::dev::Stream;
use chrono::{DateTime, Utc};
use crossbeam::atomic::AtomicConsume;
use device::Device;
use mongodb::bson::Bson;
use mongodb::{
    bson::{doc, from_document},
    options::{AggregateOptions, FindOneOptions, FindOptions},
};
use no_data_error::NoDataError;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::SystemTime;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use crate::WeatherQuery;

pub mod connection;
mod device;
mod no_data_error;

static mut DATA_PROCESSOR: Option<Arc<Mutex<DataProcessor>>> = None;

pub struct DataProcessor {
    connection: DBConnection,
    latest_weather_cache: BTreeMap<String, SensorData>,
}

impl Clone for DataProcessor {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
            latest_weather_cache: self.latest_weather_cache.clone(),
        }
    }
}

impl DataProcessor {
    async fn new() -> Self {
        Self {
            connection: DBConnection::new().await.unwrap(),
            latest_weather_cache: BTreeMap::new(),
        }
    }

    pub async fn update_cache(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let devices = self.fetch_all_devices().await?;
        println!("Devices: {:?}", devices);
        for device in devices {
            let data = self.fetch_latest_sensor_data(device.name.as_str()).await?;
            self.latest_weather_cache.insert(device.name, data);
        }
        Ok(())
    }

    pub async fn get_instance() -> Arc<Mutex<Self>> {
        let socket_server = async {
            unsafe {
                if DATA_PROCESSOR.is_none() {
                    DATA_PROCESSOR = Some(Arc::new(Mutex::new(DataProcessor::new().await)))
                }
                DATA_PROCESSOR.as_ref().unwrap()
            }
        };
        Arc::clone(socket_server.await)
    }

    pub async fn fetch_latest_sensor_data(
        &self,
        sensor_id: &str,
    ) -> Result<SensorData, Box<dyn Error + Send + Sync>> {
        let options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let collection = self.connection.collection()?;

        let result = collection
            .find_one(doc! { "sensor_id": sensor_id }, options)
            .await?;

        result.ok_or(Box::new(NoDataError))
    }

    pub async fn fetch_weather(
        &self,
        query: WeatherQuery,
    ) -> Result<Vec<SensorData>, Box<dyn Error>> {
        let statement = if query.start.is_none() && query.end.is_none() {
            doc! {
                "sensor_id": &query.device.unwrap_or(String::from(""))
            }
        } else {
            doc! {
                "timestamp": {
                    "$lt": query.end.unwrap_or(bson::DateTime::from(SystemTime::now()).to_string()),
                    "$gt": query.start.unwrap_or(String::from(""))
                },
                "sensor_id": &query.device.unwrap_or(String::from(""))
            }
        };
        let collection = self.connection.collection().unwrap();
        let mut cursor;

        let mut options = None;

        if let Some(limit) = query.limit {
            let find_options = FindOptions::builder().limit(limit).build();
            options = Some(find_options);
        }

        cursor = collection.find(statement, options).await?;

        let mut res = Vec::with_capacity(cursor.size_hint().0);

        while cursor.advance().await.unwrap() {
            res.push(cursor.deserialize_current().unwrap());
        }
        Ok(res)
    }

    pub async fn fetch_all_devices(&self) -> Result<Vec<Device>, Box<dyn Error + Send + Sync>> {
        let pipeline = vec![
            doc! {
                "$group": {
                    "_id" : "$sensor_id",
                    "timestamp" : { "$max" : "$timestamp"}
                }
            },
            doc! {
                "$project": {
                    "_id": 0,
                    "name": "$_id",
                    "timestamp": 1
                }
            },
        ];

        let options = AggregateOptions::builder().build();
        let connection = self.connection.collection()?;

        let mut cursor = connection.aggregate(pipeline, options).await?;
        let mut results: Vec<Device> = Vec::new();

        while let Ok(advance) = cursor.advance().await {
            if !advance {
                break;
            }
            results.push(from_document(cursor.deserialize_current()?)?);
        }
        Ok(results)
    }

    async fn fetch_devices_with_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<SensorData>, Box<dyn Error + Send + Sync>> {
        let filter = doc! {
            "timestamp" : {
                "$gte": Bson::Int64(start.timestamp()),
                "$lte": Bson::Int64(end.timestamp())
        }
        };

        let options = FindOptions::builder().sort(filter).build();

        let collection = self.connection.collection()?;

        let mut result = vec![];
        if let Ok(mut cursor) = collection.find(None, options).await {
            while let Ok(advance) = cursor.advance().await {
                if !advance {
                    break;
                }
                result.push(cursor.deserialize_current()?);
            }
        };
        Ok(result)
    }
}
