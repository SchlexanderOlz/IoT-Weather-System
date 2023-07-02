use actix_web::{HttpServer, App, get, Responder};
use actix_web_static_files::ResourceFiles;

mod temperature_socket;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 3000;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        let generated = generate();
        App::new()
        .service(index)
        .service(ResourceFiles::new("/", generated))
    }
    )
    .bind((ADDRESS, PORT))?
    .run()
    .await
}


#[get("/sos")]
async fn index() -> impl Responder {
    format!("This will be the index site")
}