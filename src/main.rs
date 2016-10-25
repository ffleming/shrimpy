pub mod http;
pub mod os_ffi;
use std::process::exit;
use http::server::HttpServer as HttpServer;

fn main() {
    let server = HttpServer {
        port: String::from("8080"),
        host: String::from("127.0.0.1"),
        root: String::from("src/")
    };
    unsafe {
        let pid = os_ffi::fork();
        println!("woof woof {}", pid);
        match pid {
            -1 => panic!("Fork failed!"),
            0  => server.run(),
            _  =>
                {
                    println!("Daemonizing with pid {}", pid);
                    exit(0);
                }
        }
        server.run();
    }
}
