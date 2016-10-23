pub mod http;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, BufReader};
use std::thread;
use http::request::HttpRequest as HttpRequest;

fn read_stream(mut stream: &mut TcpStream) {
    let remote_ip = stream.peer_addr().unwrap().ip();
    let mut reader = BufReader::new(&mut stream);
    let mut request = String::new();
    let req_endings = vec!(
        String::from("\x0A\x0A"),
        String::from("\x0D\x0A\x0D\x0A")
    );
    while !req_endings.iter().any(|e| request.ends_with(e)) {
        let _ = reader.read_line(&mut request);
    }
    println!("Connection from {}:", remote_ip);
    println!("{}", request);
    let req = HttpRequest::new(request);
    println!("{:?}", req);
}

fn write_stream(stream: &mut TcpStream) {
    let html = include_str!("index.html");
    let _ = stream.write("HTTP/1.0 200 OK\r\n\r\n".as_bytes());
    let _ = stream.write(html.as_bytes());
    let _ = stream.flush();
    return;
}

fn process_stream(mut stream: TcpStream) {
    read_stream(&mut stream);
    write_stream(&mut stream);
    return;
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    process_stream(stream);
                });
            }
            Err(e) => {
                println!("error {:?}\n", e);
            }
        }
    }

    drop(listener);
}
