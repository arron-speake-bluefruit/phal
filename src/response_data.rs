use crate::http_status_code::HTTPStatusCode;
use tiny_http::{ Response, ResponseBox };

pub struct ResponseData {
  pub code: HTTPStatusCode,
  pub content: String,
}

impl ResponseData {
  pub fn configure_success() -> Self {
      Self {
          code: HTTPStatusCode::OK,
          content: "Configuration completed successfullly.".to_owned()
      }
  }

  pub fn not_found() -> Self {
      Self {
          code: HTTPStatusCode::NotFound,
          content: "".to_owned(),
      }
  }

  pub fn limb_not_found() -> Self {
      Self {
          code: HTTPStatusCode::NotFound,
          content: "That limb does not exist.".to_owned()
      }
  }

  pub fn ok(content: &str) -> Self {
      Self {
          code: HTTPStatusCode::OK,
          content: content.to_owned(),
      }
  }

  pub fn bad_request(content: &str) -> Self {
      Self {
          code: HTTPStatusCode::BadRequest,
          content: content.to_owned(),
      }
  }

  pub fn method_not_allowed(content: &str) -> Self {
      Self {
          code: HTTPStatusCode::MethodNotAllowed,
          content: content.to_owned(),
      }
  }

  pub fn not_implemented(content: &str) -> Self {
      Self {
          code: HTTPStatusCode::NotImplemented,
          content: content.to_owned(),
      }
  }

  pub fn forbidden() -> Self {
      Self {
          code: HTTPStatusCode::Forbidden,
          content: "".to_owned(),
      }
  }

  pub fn site_index() -> Self {
      Self::ok("PHAL Server")
  }
}

impl Into<ResponseBox> for ResponseData {
    fn into(self) -> tiny_http::ResponseBox {
        if let HTTPStatusCode::OK = self.code {
            Response::from_string(self.content)
                .boxed()
        } else {
            let code = self.code.status_code();
            let name = self.code.name();
            let message = format!("{} {}\n{}", code, name, self.content);
            Response::from_string(message)
               .with_status_code(code)
                .boxed()
        }
    }
}