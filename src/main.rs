pub mod http;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::thread;
use http::request::HttpRequest as HttpRequest;
use http::response::HttpResponse as HttpResponse;

//TODO: move this logic into request, have new take a TcpStream
fn read_stream(mut stream: &mut TcpStream) -> Result<HttpRequest, Box<Error>> {
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
    return req;
}

fn write_stream(stream: &mut TcpStream, filename: &str) {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(why) => File::open(&"404.html").expect("couldn't find 404.html"),
        Ok(file) => file,
    };
    let mut html = String::new();
    let _ = file.read_to_string(&mut html);
    let _ = stream.write("HTTP/1.0 200 OK\r\n\r\n".as_bytes());
    let _ = stream.write(html.as_bytes());
    let _ = stream.flush();
    return;
}

fn process_stream(mut stream: TcpStream) {
    let request = read_stream(&mut stream).expect("It broke");
    let filename = format!("{}{}", "./", request.path);
    write_stream(&mut stream, &filename);
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
