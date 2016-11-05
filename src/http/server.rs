use std::thread;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::error::Error;
use std::path::{PathBuf};
use std::io::{Write, Read, BufRead, BufReader};
use http::request::HttpRequest as HttpRequest;
// use http::response::HttpResponse as HttpResponse;

#[derive(Debug)]
#[derive(Clone)]
pub struct HttpServer {
    pub port: String,
    pub host: String,
    pub root: String,
}

impl HttpServer {
    pub fn run(&self) {

        let listen_string = format!("{}:{}", self.host, self.port);
        let listen_slice = listen_string.as_str();
        let listener = TcpListener::bind(listen_slice).expect("Couldn't bind");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = self.clone();
                    let _ = thread::spawn(move|| {
                        server.process_stream(stream);
                    });
                }
                Err(e) => {
                    println!("error {:?}\n", e);
                }
            }
        }
        drop(listener);
    }

    pub fn process_stream(&self, mut stream: TcpStream) {
        let request = self.read_stream(&mut stream).expect("It broke");
        let filename = format!("{}/{}", self.root, request.path);
        println!("Serving {}", filename);
        self.write_stream(&mut stream, &filename);
        return;
    }

    //CONSIDER: move this logic into request, have new take a TcpStream
    fn read_stream(&self, mut stream: &mut TcpStream) -> Result<HttpRequest, Box<Error>> {
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

    fn write_stream(&self, stream: &mut TcpStream, filename: &str) {
        let mut path = PathBuf::from(filename);
        if path.is_dir() {
            path = path.join("index.html");
        }

        let file_404 = format!("{}/{}", self.root, "404.html");

        let mut file = match File::open(path.as_path()) {
            Err(_) => match File::open(&file_404) {
                Ok(not_found) => not_found,
                Err(err) => {
                    println!("ERROR: Couldn't find 404.html");
                    println!("{:?}", err);
                    return;
                }
            },
            Ok(file) => file,
        };
        let mut html: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut html).expect("Error reading file");
        let _ = stream.write("HTTP/1.0 200 OK\r\n\r\n".as_bytes());
        let _ = stream.write(html.as_slice());
        let _ = stream.flush();
        return;
    }
}
