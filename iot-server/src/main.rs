use server::Server;

mod server;
mod db_connection;

#[tokio::main]
async fn main() {
    let mut port: Option<String> = None;
    let mut args = std::env::args();
    args.next();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--address" => port = args.next(),
            _ => panic!("Unknown argument {}", arg),
        }
    }

    let server = Server::new(
        port.expect("Missing argument 'address'! Supply it with --address")
            .as_str(),
    )
    .await;
    server.listen().await;
}
