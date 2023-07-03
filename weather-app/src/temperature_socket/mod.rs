use actix::{Actor, Handler, Message as ActixMessage, StreamHandler, AsyncContext};
use actix_web_actors::ws::{self, Message};
use db_connection::sensor_data::SensorData;
use serde_json;
use lazy_static::lazy_static;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant}
};

mod database;

use self::database::{DataProcessor, Selecter};


pub struct TemperatureSocket {
    database: DataProcessor,
    data_cache: Mutex<Arc<Option<SensorData>>>,
    last_query: Instant,
}

impl Actor for TemperatureSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl Clone for TemperatureSocket {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            data_cache: Mutex::new(Arc::new(self.data_cache.lock().unwrap().as_ref().clone())),
            last_query: self.last_query.clone(),
        }
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
struct SensorDataMessage(Option<SensorData>);

impl Handler<SensorDataMessage> for TemperatureSocket {
    type Result = ();

    fn handle(&mut self, msg: SensorDataMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg.0).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TemperatureSocket {
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let this = self.clone();

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let data = this.get_data();
            addr.do_send(SensorDataMessage(data));
        });
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Close(_)) => {
                ctx.close(None);
                return;
            }
            Ok(Message::Ping(_)) => ctx.pong("\0".as_bytes()),
            _ => (),
        }
    }
}

impl TemperatureSocket {
    pub async fn new() -> Self {
        Self {
            data_cache: Mutex::new(Arc::new(None)),
            last_query: Instant::now(),
            database: DataProcessor::new().await,
        }
    }

    fn get_data(&self) -> Option<SensorData> {
        if Instant::now() - self.last_query > Duration::from_secs(5) {
            let data = match self.database.get_newest_temperature() {
                Ok(data) => Some(data),
                Err(err) => {
                    println!("[-]{}", err);
                    return None;
                }
            };
            *self.data_cache.lock().unwrap() = Arc::new(data.clone());
            data
        } else {
            self.data_cache.lock().unwrap().as_ref().clone()
        }
    }
}
