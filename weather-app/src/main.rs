use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_web_static_files::ResourceFiles;
use socket_server::temperature_socket::TemperatureSocket;

mod socket_server;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 3030;

async fn temperature_socket_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let socket = TemperatureSocket::new().await;
    ws::start(socket, &req, stream)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .route("/ws/", web::get().to(temperature_socket_route))
            .service(ResourceFiles::new("/", generated))
    })
    .bind((ADDRESS, PORT))
    .unwrap()
    .run()
    .await
}
