use server::Server;

mod server;

#[tokio::main]
async fn main() {
    let server = Server::new("0.0.0.0:3010").await;
    server.listen().await;
}
