use std::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub verb: String,
    pub path: String,
    pub parameters: HashMap<String, String>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new(request: String) -> Result<HttpRequest, Box<Error>> {
        let error = Err(From::from("Could not parse request"));
        let mut request_i = request.split("\n").map(|l| l.trim() );
        let request_line = match request_i.next() {
            Some(slice) => slice,
            None        => return error
        };
        let headers_str = &(request_i.collect::<Vec<&str>>().join("\n"));
        let headers = build_headers(headers_str);

        let request_parts: Vec<&str> = request_line.split(" ").collect();
        if request_parts.len() != 3 {
            return error;
        }
        let verb = request_parts[0];
        let path_with_params = request_parts[1];
        let http_version = request_parts[2];
        let mut path_iterator = path_with_params.splitn(2, "?");
        let path = match path_iterator.next() {
            Some(p) => p,
            None        => return error
        };
        let param_string = match path_iterator.next() {
            Some(params) => params,
            None    => ""
        };
        let parameters = build_parameters(param_string);

        return Ok(HttpRequest {
            verb: String::from(verb),
            path: String::from(path),
            http_version: String::from(http_version),
            parameters: parameters,
            headers: headers,
        });
    }
}

fn str_to_hash(input: &str, item_separator: &str, key_value_separator: &str)
    -> HashMap<String, String> {
    let hash: HashMap<String, String> = input.trim().split(item_separator).
        map(|kv| kv.splitn(2, key_value_separator).collect::<Vec<&str>>()).
        map(|vec| {
            let k = vec[0].trim().to_string();
            let v = if vec.len() == 2 {
                vec[1].trim().to_string()
            } else {
                "".to_string()
            };
            (k, v)
        }).collect();
    return hash;
}

fn build_headers(header_str: &str) -> HashMap<String, String> {
    return str_to_hash(header_str, "\n", ":");
}

fn build_parameters(param_str: &str) -> HashMap<String, String> {
    return str_to_hash(param_str, "&", "=");
}

