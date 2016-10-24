pub mod http;
use http::server::HttpServer as HttpServer;

fn main() {
    let server = HttpServer {
        port: String::from("8080"),
        host: String::from("127.0.0.1"),
        root: String::from("src/")
    };
    server.run();
}
