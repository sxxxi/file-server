mod server;
mod payload;
mod connection;
mod manager;


#[tokio::main]
async fn main() {
    use crate::server::serve;
    serve("0.0.0.0:8080").await.unwrap();
}
