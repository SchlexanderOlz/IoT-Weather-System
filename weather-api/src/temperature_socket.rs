use crate::database::DataProcessor;

use actix::{Actor, AsyncContext, Handler, Message as ActixMessage, StreamHandler};
use actix_web_actors::ws::{self, Message};
use db_connection::sensor_data::SensorData;
use serde_json;
use std::{sync::Arc, thread, time::Duration};

pub struct TemperatureSocket {
    database: Arc<DataProcessor>,
    device_name: String,
}

impl Actor for TemperatureSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl Clone for TemperatureSocket {
    fn clone(&self) -> Self {
        Self {
            database: Arc::clone(&self.database),
            device_name: self.device_name.clone(),
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

        let db = Arc::clone(&self.database);
        let device_name = self.device_name.clone();

        actix::spawn(async move {
            loop {
                let data = db.fetch_latest_sensor_data(device_name.as_str()).await.ok();
                addr.do_send(SensorDataMessage(data));
                thread::sleep(Duration::from_secs(1));
            }
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
    pub async fn new(device_name: String, database: Arc<DataProcessor>) -> Self {
        Self {
            database,
            device_name,
        }
    }
}
