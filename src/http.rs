#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    QUERY,
    HEAD,
    DELETE,
}

impl HttpMethod {
    pub fn from(value: &str) -> HttpMethod {
        match value {
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "QUERY" => HttpMethod::QUERY,
            "HEAD" => HttpMethod::HEAD,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::QUERY => "QUERY",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

pub enum HttpStatus {
    OK,
}

impl HttpStatus {
    pub fn name(&self) -> &str {
        match self {
            HttpStatus::OK => "OK",
        }
    }

    pub fn code(&self) -> i32 {
        match self {
            HttpStatus::OK => 200,
        }
    }
}