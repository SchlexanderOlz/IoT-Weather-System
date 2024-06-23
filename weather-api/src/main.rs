use actix_cors::Cors;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use database::DataProcessor;
use mongodb::bson::DateTime;
use serde::Deserialize;
use temperature_socket::TemperatureSocket;

mod database;
mod temperature_socket;

async fn hello() -> impl Responder {
    "Hello, world!"
}

async fn weather_socket(
    req: HttpRequest,
    stream: web::Payload,
    device_name: web::Path<(String, )>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("{:?}", device_name.clone());
    let socket = TemperatureSocket::new(
        device_name.into_inner().0,
        DataProcessor::get_instance().await,
    )
    .await;
    ws::start(socket, &req, stream)
}

async fn get_devices() -> impl Responder {
    println!("Getting devices");
    match DataProcessor::get_instance()
        .await
        .lock()
        .await
        .fetch_all_devices()
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub limit: Option<i64>,
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
}

async fn get_weather_data(query: web::Query<WeatherQuery>) -> impl Responder {
    let query = query.into_inner();

    let data = DataProcessor::get_instance()
        .await
        .lock()
        .await
        .fetch_weather(query)
        .await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();

    let handle = actix::spawn(async {
        loop {
            DataProcessor::get_instance()
                .await
                .lock()
                .await
                .update_cache()
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let address = std::env::var("HOST_ADDRESS").expect("HOST_ADDRESS not set");
    let port: u16 = std::env::var("HOST_PORT")
        .expect("HOST_PORT not set")
        .parse()
        .unwrap();

    println!("Starting server on {}:{}", address, port);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/devices", web::get().to(get_devices))
            .route("/weather", web::get().to(get_weather_data))
            .route("/weather/{device_name}", web::get().to(weather_socket))
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}

