#[derive(Clone, Copy)]
pub enum HTTPStatusCode {
    OK,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotImplemented,
}

impl HTTPStatusCode {
    pub fn status_code(&self) -> u16 {
        use HTTPStatusCode::*;
        match self {
            OK => 200,
            BadRequest => 400,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotImplemented => 501,
        }
    }

    pub fn name(&self) -> &'static str {
        use HTTPStatusCode::*;
        match self {
            OK => "OK",
            BadRequest => "Bad request",
            Forbidden => "Forbidden",
            NotFound => "Not found",
            MethodNotAllowed => "Method not allowed",
            NotImplemented => "Not implemented",
        }
    }
}