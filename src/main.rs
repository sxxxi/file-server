mod server;
mod payload;
mod connection;
mod core;


#[tokio::main]
async fn main() {
    // use crate::server::serve;
    // serve("0.0.0.0:8080").await.unwrap();

    use crate::core::AppSpace;
    let mut x = AppSpace::new("/home/domino/Projects");
    x.current = x.parse_path("/home/domino/Projects/Linux/Rust/resp_impl/../tokio_ghoul").unwrap();

    println!("{:?}", x.pwd());


    println!("{:?}", x.parse_path("~/..").unwrap());


}
