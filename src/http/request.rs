#[derive(Debug)]
pub struct HttpRequest {
    verb: String,
    path: String,
    parameters: String,
    http_version: String,
    hostname: String,
    headers: Vec<String>,
    body: String,
}
impl HttpRequest {
    pub fn new(request: String) -> HttpRequest {
        let mut iter = request.split("\x0D\x0A").map(|l| l.trim() );
        let first_line = iter.next().unwrap();
        let verb = first_line.splitn(2, " ").next().unwrap();
        let mut iter = first_line.rsplitn(2, " ");
        let http_version = iter.next().unwrap().splitn(2, "HTTP/").last().unwrap();
        let mut path_i = iter.next().unwrap().splitn(2, "?");
        let path = path_i.next().unwrap();
        let parameters = path_i.next().unwrap();
        return HttpRequest {
            verb: String::from(verb),
            path: String::from(path),
            parameters: String::from(parameters),
            http_version: String::from(http_version),
            hostname: String::from("hostname"),
            headers: vec!(String::from("header1"), String::from("header2")),
            body: String::from("body"),
        };
    }
}

