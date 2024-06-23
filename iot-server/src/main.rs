use server::Server;

mod db_connection;
mod server;

#[tokio::main]
async fn main() {
    let url: String = std::env::var("HOST_URL").expect("HOST_URL not set");

    let server = Server::new(&url).await;
    server.listen().await;
}
