use server::Server;

mod server;

#[tokio::main]
async fn main() {
    let server = Server::new("0.0.0.0:3000").await;
    server.listen().await;
}
