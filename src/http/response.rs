use std::error::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub code: String,
    pub code_mnemonic: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl HttpResponse {
    pub fn new(http_version: &String, body_string: &String) -> Result<HttpResponse, Box<Error>> {
        let code = String::from("200");
        return Ok(HttpResponse {
            code: code,
            code_mnemonic: String::from("OK"),
            http_version: http_version.clone(),
            headers: HashMap::new(),
            body: body_string.clone()
        });
    }
    pub fn as_string(&self) -> String {
        format!("{} {} {}\r\n", self.http_version, self.code, self.code_mnemonic)
    }
}
