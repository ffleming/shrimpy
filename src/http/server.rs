use std::thread;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::error::Error;
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::io::{Write, Read, BufRead, BufReader};
use http::request::HttpRequest as HttpRequest;
use http::response::HttpResponse as HttpResponse;

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
                    let handle = thread::spawn(move|| {
                        server.process_stream(stream);
                    });
                    handle.join().unwrap();
                }
                Err(e) => {
                    println!("error {:?}\n", e);
                }
            }
        }
        drop(listener);
    }

    pub fn process_stream(&self, mut stream: TcpStream) {
        let request = self.read_stream(&mut stream).expect("Request broke");
        let filename = self.requested_absolute_path(&request);
        let body_string = self.read_file(filename);
        let response = HttpResponse::new( &request.http_version,
                                          &body_string
                                        ).expect("Response broke");
        self.write_response(&mut stream, &response);
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
        let req = HttpRequest::new(request);
        return req;
    }

    fn read_file(&self, filename: PathBuf) -> String {
        let mut result = String::new();
        println!("Serving {:?}", filename);
        match File::open(&filename) {
            Err(_) => { result = self.error_404(&filename) },
            Ok(mut file) => { file.read_to_string(&mut result).expect("Died in read_file"); },
        }
        result
    }

    fn requested_absolute_path(&self, request: &HttpRequest) -> PathBuf {
        // treat requested path as absolute to limit directory traversal,
        // then remove extraneous path separator
        let single = &(MAIN_SEPARATOR.to_string());
        let double = &(format!("{}{}", single, single));
        let filename = format!("{}{}", self.root, request.path).replace(double, single);
        let mut pathbuf = PathBuf::from(filename);
        if pathbuf.is_dir() {
            pathbuf = pathbuf.join("index.html");
        };
        pathbuf
    }

    fn write_response(&self, stream: &mut TcpStream, response: &HttpResponse) {
        let _ = stream.write("HTTP/1.0 200 OK\r\n\r\n".as_bytes());
        let _ = stream.write(response.body.as_bytes());
        let _ = stream.flush();
    }

    fn error_404(&self, filename: &PathBuf) -> String {
        format!(
            "
            <html>
            <head><title>Shrimpy - Not found</title></head>
            <body><h1>404 - Not Found</h1>The file {:?} could not be found</body>
            ", filename)
    }
}
