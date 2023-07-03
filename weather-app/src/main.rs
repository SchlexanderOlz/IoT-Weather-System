use actix_web::{HttpServer, App,  web, HttpRequest, HttpResponse};
use actix_web_static_files::ResourceFiles;
use actix_web_actors::ws;
use temperature_socket::TemperatureSocket;

mod temperature_socket;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 3030;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let generated = generate();
        App::new()
        .route("/ws/", web::get().to(temperature_socket_route))
        .service(ResourceFiles::new("/", generated))
    }
    )
    .bind((ADDRESS, PORT))?
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
