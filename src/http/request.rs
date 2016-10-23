use std::error::Error;

#[derive(Debug)]
pub struct HttpRequest {
    verb: String,
    path: String,
    parameters: String,
    http_version: String,
    headers: Vec<String>,
    body: String,
}
impl HttpRequest {
    pub fn new(request: String) -> Result<HttpRequest, Box<Error>> {
        let error = Err(From::from("Could not parse request"));
        let mut iter = request.split("\x0D\x0A").map(|l| l.trim() );

        let first_line = match iter.next() {
            Some(slice) => slice,
            None        => return error
        };
        let words: Vec<&str> = first_line.split(" ").collect();
        if words.len() != 3 {
            return error;
        }
        let verb = words[0];
        let path = words[1];
        let http_version = words[2];
        let mut path_iterator = path.splitn(2, "?");
        let path = match path_iterator.next() {
            Some(p) => p,
            None        => return error
        };
        let parameters = match path_iterator.next() {
            Some(params) => params,
            None    => ""
        };
        return Ok(HttpRequest {
            verb: String::from(verb),
            path: String::from(path),
            parameters: String::from(parameters),
            http_version: String::from(http_version),
            headers: vec!(String::from("header1"), String::from("header2")),
            body: String::from("body"),
        });
    }
}

