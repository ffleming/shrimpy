use std::error::Error;
use std::collections::HashMap;
use http::request::HttpRequest as HttpRequest;

#[derive(Debug)]
pub struct HttpResponse {
    pub code: String,
    pub code_mnemonic: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl HttpResponse {
    pub fn new(request: HttpRequest, body_string: String) -> Result<HttpResponse, Box<Error>> {
        let code = String::from("200");
        let http_version = request.http_version;
        return Ok(HttpResponse {
            code: code,
            code_mnemonic: String::from("OK"),
            http_version: http_version,
            headers: HashMap::new(),
            body: body_string
        });
    }
    pub fn as_string(&self) -> String {
        let response_line = format!("{} {} {}\r\n",
                                    self.http_version, self.code, self.code_mnemonic);

        return response_line;
    }
}
