use std::net::TcpListener;

fn main() {
    println!("Hello, world!");

    start();
}

fn start() {
    let port = "8080";
    println!("Serving Server on port {port}");

    let listener = TcpListener::bind("127.0.0.1:".to_owned() + port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
