use server::Server;

mod server;


#[tokio::main]
async fn main() {
    let server = Server::new("127.0.0.1:3000").await;
    server.listen();
}
