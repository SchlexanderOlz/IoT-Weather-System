use actix::dev::Stream;
use chrono::{DateTime, Utc};
use db_connection::{
    sensor_data::SensorData, DBConnection,
};
use device::Device;
use mongodb::bson::Bson;
use mongodb::{
    bson::{doc, from_document},
    options::{AggregateOptions, FindOneOptions, FindOptions},
};
use no_data_error::NoDataError;
use std::{error::Error, sync::Arc};

use crate::WeatherQuery;

mod device;
mod no_data_error;

static mut DATA_PROCESSOR: Option<Arc<DataProcessor>> = None;

pub struct DataProcessor {
    connection: DBConnection,
}

impl Clone for DataProcessor {
    fn clone(&self) -> Self {
        Self {
            connection: self.connection.clone(),
        }
    }
}

impl DataProcessor {
    async fn new() -> Self {
        Self {
            connection: DBConnection::new().await.unwrap(),
        }
    }

    pub async fn get_instance() -> Arc<Self> {
        let socket_server = async {
            unsafe {
                if DATA_PROCESSOR.is_none() {
                    DATA_PROCESSOR = Some(Arc::new(DataProcessor::new().await))
                }
                DATA_PROCESSOR.as_ref().expect("")
            }
        };
        Arc::clone(socket_server.await)
    }

    pub async fn fetch_latest_sensor_data(&self) -> Result<SensorData, Box<dyn Error + Send + Sync>> {
        let options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let collection = self.connection.collection();
        if collection.is_err() {
            return Err(collection.err().unwrap());
        }
        let collection = collection.unwrap();

        let result = collection.find_one(None, options).await?;

        result.ok_or(Box::new(NoDataError))
    }

    pub async fn fetch_weather(
        &self,
        query: WeatherQuery,
    ) -> Result<Vec<SensorData>, Box<dyn Error>> {
        let statement = doc! {
            "timestamp": {
                "$lt": query.end.unwrap_or(bson::DateTime::from_millis(i64::MAX)),
                "$gt": query.start
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
