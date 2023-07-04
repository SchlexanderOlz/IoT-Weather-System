use actix::{Actor, Handler, Message as ActixMessage, StreamHandler, AsyncContext};
use actix_web_actors::ws::{self, Message};
use db_connection::sensor_data::SensorData;
use serde_json;
use std::{
    sync::{Mutex},
    thread,
    time::{Duration}
};
use super::{SocketServer, get_instance_anyways};


pub mod database;

pub struct TemperatureSocket {
    server: Mutex<&'static SocketServer>
}

impl Actor for TemperatureSocket {
    type Context = ws::WebsocketContext<Self>;
}


impl Clone for TemperatureSocket {
    fn clone(&self) -> Self {
        Self { server: Mutex::new(self.server.lock().unwrap().clone()) }
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
        let self_clone = self.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(5));
            let server = self_clone.server.lock().unwrap();
            let data = server.get_data();
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
        Self { server: Mutex::new(get_instance_anyways().await) }
    }
}
