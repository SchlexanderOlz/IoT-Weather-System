use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use database::{DataProcessor, Selecter};
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
            .route("/ws/", web::get().to(temperature_socket_route))
            .route("/getDevices", web::get().to(get_devices))
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
    match processor.get_all_devices().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
