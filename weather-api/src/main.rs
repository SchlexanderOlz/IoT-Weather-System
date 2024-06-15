use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use database::DataProcessor;
use mongodb::bson::DateTime;
use serde::Deserialize;
use socket_server::temperature_socket::TemperatureSocket;

mod database;
mod socket_server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut address: Option<String> = None;
    let mut port: Option<u16> = None;
    let mut args = std::env::args();
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--address" => address = args.next(),
            "--port" => {
                port = args
                    .next()
                    .expect("Missing value after port argument")
                    .parse()
                    .ok()
            }
            _ => panic!("Unknown argument {}", arg),
        }
    }

    HttpServer::new(|| {
        App::new()
            .route("/weather/", web::get().to(temperature_socket_route))
            .route("/devices/", web::get().to(get_devices))
    })
    .bind((
        address
            .expect("Address value was not supplied with --address")
            .as_str(),
        port.expect("Port value was not supplied with --port"),
    ))
    .unwrap()
    .run()
    .await
}

async fn temperature_socket_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let socket = TemperatureSocket::new().await;
    ws::start(socket, &req, stream)
}

async fn get_devices() -> impl Responder {
    let processor = DataProcessor::get_instance().await;
    match processor.fetch_all_devices().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
struct WeatherQuery {
    pub limit: Option<i64>,
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
}

#[actix_web::get("/weather/")]
async fn get_weather_data(query: web::Query<WeatherQuery>) -> impl Responder {
    let query = query.into_inner();

    let data = DataProcessor::get_instance().await.fetch_weather(query).await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
